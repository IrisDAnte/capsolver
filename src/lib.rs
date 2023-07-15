use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

const SUPPORTED_MODULES: [&'static str; 2] = ["common", "queueit"];
const H_CAPTCHA_TYPES: [&'static str; 3] =
    ["HCaptchaTask", "HCaptchaTaskProxyLess", "HCaptchaTurboTask"];
const GEE_TEST_TYPES: [&'static str; 2] = ["GeeTestTask", "GeeTestTaskProxyLess"];
const RE_CAPTCHA_V2_TYPES: [&'static str; 2] = ["ReCaptchaV2Task", "ReCaptchaV2TaskProxyLess"];
const RE_CAPTCHA_V3_TYPES: [&'static str; 2] = ["ReCaptchaV3Task", "ReCaptchaV3TaskProxyLess"];
const MT_CAPTCHA_TYPES: [&'static str; 2] = ["MtCaptchaTask", "MtCaptchaTaskProxyLess"];
const AWS_WAF_TYPES: [&'static str; 2] = ["AwsWafTask", "AwsWafTaskProxyLess"];
const CYBER_SI_ARA_TYPES: [&'static str; 2] = ["AwsWafTask", "AwsWafTaskProxyLess"];

#[derive(Clone)]
pub struct Config {
    api_key: String,
    api_url: Url,
    client: Client,
}

impl Config {
    pub fn new(api_key: &str, api_url: Option<&str>) -> Self {
        let client = Client::new();
        let api_key = api_key.to_string();
        let api_url = Url::parse(api_url.unwrap_or("https://api.capsolver.com")).unwrap();

        Self {
            api_url,
            api_key,
            client,
        }
    }

    fn make_body(&self) -> Value {
        json!({
            "clientKey": self.api_key
        })
    }

    async fn create_task(&self, body: Value) -> Result<Value, String> {
        let res = self
            .client
            .post(self.api_url.join("createTask").unwrap())
            .json(&body)
            .send()
            .await;
        println!("{}", body.to_string());

        match res {
            Ok(o) => {
                let data = json!(o.text().await.unwrap());

                if data["errorId"].as_i64().unwrap_or(0) != 0 {
                    return Err(format!(
                        "{}: {}",
                        data["errorCode"].as_str().unwrap(),
                        data["errorDescription"].as_str().unwrap()
                    ));
                }

                Ok(data)
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalance {
    error_id: Option<usize>,
    error_code: Option<String>,
    error_description: Option<String>,
    pub balance: Option<f64>,
    pub packages: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResponse {
    error_id: Option<usize>,
    error_code: Option<String>,
    error_description: Option<String>,
    pub status: Option<String>,
    pub task_id: Option<String>,
}

pub struct CapSolver {
    config: Config,
    recognition: Recognition,
    token: Token,
}

#[derive(Deserialize, Serialize)]
pub struct SolveOptions<'a> {
    #[serde(rename = "type")]
    task_type: &'a str,
    body: &'a str,
    module: &'a str,
}

impl CapSolver {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            recognition: Recognition {
                config: config.clone(),
            },
            token: Token { config },
        }
    }

    pub async fn create_task(&self, body: &str) -> Result<Value, String> {
        match serde_json::from_str(body) {
            Ok(o) => self.config.create_task(o).await,
            _ => Err("Inavlid JSON".to_string()),
        }
    }

    pub fn recognition(&self) -> &Recognition {
        &self.recognition
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub async fn get_balance(&self) -> Result<GetBalance, String> {
        let config = &self.config;
        let body = config.make_body();

        let res = config
            .client
            .post(config.api_url.join("getBalance").unwrap())
            .json(&body)
            .send()
            .await;

        match res {
            Ok(o) => {
                let data = o.json::<GetBalance>().await.unwrap();

                if data.error_id.unwrap() != 0 {
                    return Err(format!(
                        "{} {}",
                        data.error_code.unwrap(),
                        data.error_description.unwrap()
                    ));
                }

                Ok(data)
            }
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn get_task_result(&self, task_id: &str) -> Result<Value, String> {
        let config = &self.config;
        let mut body = config.make_body();

        body["taskId"] = json!(task_id);

        let res = config
            .client
            .post(config.api_url.join("getTaskResult").unwrap())
            .json(&body)
            .send()
            .await;
        println!("{}", body.to_string());

        match res {
            Ok(o) => {
                let data = json!(o.text().await.unwrap());

                if data["errorId"].as_i64().unwrap_or(0) != 0 {
                    return Err(format!(
                        "{}: {}",
                        data["errorCode"].as_str().unwrap(),
                        data["errorDescription"].as_str().unwrap()
                    ));
                }

                Ok(data)
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

pub struct Recognition {
    config: Config,
}

impl Recognition {
    pub async fn image_to_text(
        &self,
        img: String,
        module: Option<&str>,
        score: Option<f64>,
        case_sensitive: Option<bool>,
    ) -> Result<Value, String> {
        let config = &self.config;
        let mut body = config.make_body();
        let task = &mut body["task"];

        task["type"] = json!("ImageToTextTask");
        task["body"] = json!(img);

        match module {
            Some(m) => {
                if !SUPPORTED_MODULES.contains(&m) {
                    return Err("Unsupported module".to_string());
                }

                task["module"] = json!(module);
            }
            _ => {}
        }

        match score {
            Some(s) => {
                if s > 1.0 || s < 0.8 {
                    return Err("Score must be within 0.8 ~ 1".to_string());
                }

                task["score"] = json!(s);
            }
            _ => {}
        };

        task["case"] = json!(case_sensitive.unwrap_or(false));

        config.create_task(body).await
    }

    pub async fn h_captcha(&self, queries: Vec<String>, question: &str) -> Result<Value, String> {
        let config = &self.config;
        let mut body = config.make_body();
        let task = &mut body["task"];

        task["type"] = json!("HCaptchaClassification");
        task["queries"] = json!(queries);
        task["question"] = json!(question);

        config.create_task(body).await
    }

    pub async fn fun_captcha(&self, imgs: Vec<String>, question: &str) -> Result<Value, String> {
        let config = &self.config;
        let mut body = config.make_body();
        let task = &mut body["task"];

        task["type"] = json!("FunCaptchaClassification");
        task["images"] = json!(imgs);
        task["question"] = json!(question);

        config.create_task(body).await
    }

    pub async fn re_captcha(&self, img: String, question: &str) -> Result<Value, String> {
        let config = &self.config;
        let mut body = config.make_body();
        let task = &mut body["task"];

        task["type"] = json!("ReCaptchaV2Classification");
        task["image"] = json!(img);
        task["question"] = json!(question);

        config.create_task(body).await
    }

    pub async fn aws_waf(&self, imgs: Vec<String>, question: &str) -> Result<Value, String> {
        let config = &self.config;
        let mut body = config.make_body();
        let task = &mut body["task"];

        task["type"] = json!("AwsWafClassification");
        task["images"] = json!(imgs);
        task["question"] = json!(question);

        config.create_task(body).await
    }
}

pub struct Token {
    config: Config,
}

impl Token {
    pub async fn h_captcha(
        &self,
        r#type: &str,
        website_url: &str,
        website_key: &str,
        is_invisible: Option<bool>,
        proxy: Option<String>,
        enterprise_payload: Option<HashMap<&str, String>>,
        user_agent: Option<&str>,
    ) -> Result<Value, String> {
        let config = &self.config;
        let mut body = config.make_body();
        let task = &mut body["task"];

        task["type"] = json!("ImageToTextTask");
        task["websiteURL"] = json!(website_url);
        task["websiteKey"] = json!(website_key);

        if is_invisible.is_some() {
            task["isInvisible"] = json!(is_invisible.unwrap());
        }

        if proxy.is_some() {
            task["proxy"] = json!(proxy.unwrap());
        }

        if enterprise_payload.is_some() {
            task["enterprisePayload"] = json!(enterprise_payload.unwrap());
        }

        if user_agent.is_some() {
            task["userAgent"] = json!(user_agent.unwrap());
        }

        if !H_CAPTCHA_TYPES.contains(&r#type) {
            return Err("Unsupported type".to_string());
        }

        task["type"] = json!(r#type);

        config.create_task(body).await
    }

    pub async fn fun_captcha(
        &self,
        website_url: &str,
        website_public_key: &str,
        fun_captcha_api_js_subdomain: Option<String>,
        data: Option<String>,
        proxy: Option<String>,
    ) -> Result<Value, String> {
        let config = &self.config;
        let mut body = config.make_body();
        let task = &mut body["task"];

        task["type"] = json!("FunCaptchaTaskProxyLess");
        task["websiteURL"] = json!(website_url);
        task["websitePublicKey"] = json!(website_public_key);

        if fun_captcha_api_js_subdomain.is_some() {
            task["funcaptchaApiJSSubdomain"] = json!(fun_captcha_api_js_subdomain.unwrap());
        }

        if proxy.is_some() {
            task["proxy"] = json!(proxy.unwrap());
        }

        if data.is_some() {
            task["data"] = json!(data.unwrap());
        }

        config.create_task(body).await
    }

    pub async fn gee_test(
        &self,
        r#type: &str,
        website_url: &str,
        gt: Option<String>,
        challenge: Option<String>,
        captcha_id: Option<String>,
        gee_test_api_server_subdomain: Option<String>,
        proxy: Option<String>,
    ) -> Result<Value, String> {
        let config = &self.config;
        let mut body = config.make_body();
        let task = &mut body["task"];

        task["websiteURL"] = json!(website_url);

        if gt.is_some() {
            task["gt"] = json!(gt.unwrap());
        }

        if challenge.is_some() {
            task["challenge"] = json!(challenge.unwrap());
        }

        if captcha_id.is_some() {
            task["captchaId"] = json!(captcha_id.unwrap());
        }

        if proxy.is_some() {
            task["proxy"] = json!(proxy.unwrap());
        }

        if gee_test_api_server_subdomain.is_some() {
            task["geetestApiServerSubdomain"] = json!(gee_test_api_server_subdomain.unwrap());
        }

        if !GEE_TEST_TYPES.contains(&r#type) {
            return Err("Unsupported type".to_string());
        }

        task["type"] = json!(r#type);

        config.create_task(body).await
    }

    pub async fn re_captcha_v2(
        &self,
        r#type: &str,
        website_url: &str,
        website_key: &str,
        proxy: Option<String>,
        page_action: Option<String>,
        enterprise_payload: Option<HashMap<&str, String>>,
        is_invisible: Option<bool>,
        api_domain: Option<String>,
        user_agent: Option<&str>,
        cookies: Option<Vec<HashMap<String, String>>>,
        anchor: Option<String>,
        reload: Option<String>,
    ) -> Result<Value, String> {
        let config = &self.config;
        let mut body = config.make_body();
        let task = &mut body["task"];

        task["websiteURL"] = json!(website_url);
        task["websiteKey"] = json!(website_key);

        if page_action.is_some() {
            task["pageAction"] = json!(page_action.unwrap());
        }

        if api_domain.is_some() {
            task["apiDomain"] = json!(api_domain.unwrap());
        }

        if cookies.is_some() {
            task["cookies"] = json!(cookies.unwrap());
        }

        if anchor.is_some() {
            task["anchor"] = json!(anchor.unwrap());
        }

        if reload.is_some() {
            task["reload"] = json!(reload.unwrap());
        }

        if is_invisible.is_some() {
            task["isInvisible"] = json!(is_invisible.unwrap());
        }

        if proxy.is_some() {
            task["proxy"] = json!(proxy.unwrap());
        }

        if enterprise_payload.is_some() {
            task["enterprisePayload"] = json!(enterprise_payload.unwrap());
        }

        if user_agent.is_some() {
            task["userAgent"] = json!(user_agent.unwrap());
        }

        if !RE_CAPTCHA_V2_TYPES.contains(&r#type) {
            return Err("Unsupported type".to_string());
        }

        task["type"] = json!(r#type);

        config.create_task(body).await
    }

    pub async fn re_captcha_v3(
        &self,
        r#type: &str,
        website_url: &str,
        website_key: &str,
        proxy: Option<String>,
        min_score: Option<f64>,
        page_action: String,
        enterprise_payload: Option<HashMap<&str, String>>,
        api_domain: Option<String>,
        user_agent: Option<&str>,
        cookies: Option<Vec<HashMap<String, String>>>,
        anchor: Option<String>,
        reload: Option<String>,
    ) -> Result<Value, String> {
        let config = &self.config;
        let mut body = config.make_body();
        let task = &mut body["task"];

        task["websiteURL"] = json!(website_url);
        task["websiteKey"] = json!(website_key);
        task["pageAction"] = json!(page_action);

        if min_score.is_some() {
            task["minScore"] = json!(min_score.unwrap())
        }

        if api_domain.is_some() {
            task["apiDomain"] = json!(api_domain.unwrap());
        }

        if cookies.is_some() {
            task["cookies"] = json!(cookies.unwrap());
        }

        if anchor.is_some() {
            task["anchor"] = json!(anchor.unwrap());
        }

        if reload.is_some() {
            task["reload"] = json!(reload.unwrap());
        }

        if proxy.is_some() {
            task["proxy"] = json!(proxy.unwrap());
        }

        if enterprise_payload.is_some() {
            task["enterprisePayload"] = json!(enterprise_payload.unwrap());
        }

        if user_agent.is_some() {
            task["userAgent"] = json!(user_agent.unwrap());
        }

        if !RE_CAPTCHA_V3_TYPES.contains(&r#type) {
            return Err("Unsupported type".to_string());
        }

        task["type"] = json!(r#type);

        config.create_task(body).await
    }

    pub async fn mt_captcha(
        &self,
        r#type: &str,
        website_url: &str,
        website_key: &str,
        proxy: Option<String>,
    ) -> Result<Value, String> {
        let config = &self.config;
        let mut body = config.make_body();
        let task = &mut body["task"];

        task["websiteURL"] = json!(website_url);
        task["websiteKey"] = json!(website_key);

        if proxy.is_some() {
            task["proxy"] = json!(proxy.unwrap());
        }

        if !MT_CAPTCHA_TYPES.contains(&r#type) {
            return Err("Unsupported type".to_string());
        }

        task["type"] = json!(r#type);

        config.create_task(body).await
    }

    pub async fn datadome(
        &self,
        website_url: &str,
        captcha_url: &str,
        proxy: String,
        user_agent: &str,
    ) -> Result<Value, String> {
        let config = &self.config;
        let mut body = config.make_body();

        body["task"] = json!({
            "type": "DataDomeSliderTask",
            "websiteURL": website_url,
            "captchaUrl": captcha_url,
            "proxy": proxy,
            "userAgent": user_agent
        });

        config.create_task(body).await
    }

    pub async fn aws_waf(
        &self,
        r#type: &str,
        website_url: &str,
        proxy: Option<String>,
    ) -> Result<Value, String> {
        let config = &self.config;
        let mut body = config.make_body();
        let task = &mut body["task"];

        task["websiteURL"] = json!(website_url);

        if proxy.is_some() {
            task["proxy"] = json!(proxy.unwrap());
        }

        if !AWS_WAF_TYPES.contains(&r#type) {
            return Err("Unsupported type".to_string());
        }

        task["type"] = json!(r#type);

        config.create_task(body).await
    }

    pub async fn cyber_si_ara(
        &self,
        r#type: &str,
        slide_master_url_id: &str,
        website_url: &str,
        user_agent: &str,
        proxy: Option<String>,
    ) -> Result<Value, String> {
        let config = &self.config;
        let mut body = config.make_body();

        if !CYBER_SI_ARA_TYPES.contains(&r#type) {
            return Err("Unsupported type".to_string());
        }

        body["task"] = json!({
            "type": r#type,
            "SlideMasterURLId": slide_master_url_id,
            "websiteURL": website_url,
            "userAgent": user_agent
        });

        if proxy.is_some() {
            body["task"]["proxy"] = json!(proxy.unwrap());
        }

        config.create_task(body).await
    }

    pub async fn cloudfare_turnstile(
        &self,
        website_url: &str,
        website_key: &str,
        metadata: HashMap<&str, &str>,
        proxy: &str,
    ) -> Result<Value, String> {
        let config = &self.config;
        let mut body = config.make_body();

        body["task"] = json!({
            "type": "AntiCloudflareTask",
            "websiteURL": website_url,
            "websiteKey": website_key,
            "metadata": metadata,
            "proxy": proxy
        });

        config.create_task(body).await
    }

    pub async fn cloudfare_challange(
        &self,
        website_url: &str,
        metadata: HashMap<&str, &str>,
        html: &str,
        proxy: &str,
    ) -> Result<Value, String> {
        let config = &self.config;
        let mut body = config.make_body();

        body["task"] = json!({
            "type": "AntiCloudflareTask",
            "websiteURL": website_url,
            "html": html,
            "metadata": metadata,
            "proxy": proxy
        });

        config.create_task(body).await
    }
}
