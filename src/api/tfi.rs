mod tfi {
    use serde::{Deserialize, Serialize};
    use reqwest::{self, header};

    mod response {
        use super::*;

        fn default_value() -> String {
            "-".to_string()
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct TFIBase<T> {
            pub is_success: bool,
            pub status_code: i16,
            pub message: String,
            pub path: String,
            pub payload: T,
        }
    
        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct TokenPayload {
            pub token: String,
            pub token_scheme: String,
        }
    
        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct UserDataPayload {
            pub user_id: String,
            pub email: String,
            pub fullname: String,
            pub gender: String,
            pub location: String,
            pub phone_number: String,
            pub is_admin: bool,
            pub campus: String,
            pub campus_name: String,
            pub faculty: Faculty,
            pub study_program: StudyProgram,
            pub gpa: f32,
            pub semester: i8
        }
    
        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Faculty {
            pub id: String,
            pub name: String,
        }
    
        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct StudyProgram {
            pub id: String,
            pub name: String,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct SocialEventPayload {
            #[serde(alias = "iTotalRecords")] pub i_total_records: i32,
            #[serde(alias = "iTotalDisplayRecords")] pub total_display_records: i16,
            #[serde(alias = "aaData")] pub data: Vec<SocialEvent>,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct SocialEvent {
            pub id: String,
            pub id_attendance: String,
            pub selection_type: i8,
            pub status: i8,
            pub start_registration: String,
            pub end_registration: String,
            pub evaluation_due_date: String,
            pub event_category: EventCategory,
            pub scale: Scale,
            pub name: String,
            pub start_event_date: String,
            pub end_event_date: String,
            pub student_registration_date: String,
            pub date_submitted: String,
            pub date_approved: String,
            #[serde(default = "default_value")] pub organizer: String,
            #[serde(default = "default_value")] pub role: String,
            pub impactful: bool,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct EventCategory {
            pub id: i16,
            pub name: String,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Scale {
            pub id: i16,
            pub name: String,
        }
    }

    pub mod request {
        use super::*;

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Login {
            pub email: String,
        }
    }

    pub struct TfiApi {
        client: reqwest::Client
    }

    impl TfiApi {
        pub fn new() -> Self {
            let mut headers = header::HeaderMap::new();
            headers.insert(reqwest::header::ACCEPT, reqwest::header::HeaderValue::from_static("application/json"));
            headers.insert(reqwest::header::ACCEPT_ENCODING, reqwest::header::HeaderValue::from_static("gzip, defalte, br"));
            headers.insert(reqwest::header::ACCEPT_LANGUAGE, reqwest::header::HeaderValue::from_static("en-US,en;q=0.5"));
            headers.insert(reqwest::header::AUTHORIZATION, reqwest::header::HeaderValue::from_static("Bearer"));
            headers.insert(reqwest::header::CACHE_CONTROL, reqwest::header::HeaderValue::from_static("no-cache"));
            headers.insert(reqwest::header::CONNECTION, reqwest::header::HeaderValue::from_static("keep-alive"));
            // headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"));
            headers.insert(reqwest::header::DNT, reqwest::header::HeaderValue::from_static("1"));
            headers.insert(reqwest::header::HOST, reqwest::header::HeaderValue::from_static("tfiapi.apps.binus.ac.id"));
            headers.insert(reqwest::header::ORIGIN, reqwest::header::HeaderValue::from_static("https://tfi.apps.binus.ac.id"));
            headers.insert(reqwest::header::PRAGMA, reqwest::header::HeaderValue::from_static("no-cache"));
            headers.insert(reqwest::header::REFERER, reqwest::header::HeaderValue::from_static("https://tfi.apps.binus.ac.id/"));
            headers.insert(reqwest::header::TE, reqwest::header::HeaderValue::from_static("trailers"));
            headers.insert(reqwest::header::ACCEPT_ENCODING, reqwest::header::HeaderValue::from_static("gzip, defalte, br"));
            headers.insert("timezone", reqwest::header::HeaderValue::from_static("UTC+07:00"));

            let client = reqwest::Client::builder()
                .default_headers(headers)
                .no_proxy()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/12.246")
                .build()
                .unwrap();

            Self { client } 
        }

        pub async fn login(&self, email: &str) -> Result<response::TFIBase<response::TokenPayload>, reqwest::Error> {
            let request_body = request::Login { email: email.to_string() };
    
            let token = self.client
                .post("https://tfiapi.apps.binus.ac.id/api/auth/login")
                .query(&[("culture", "id-ID")])
                .json::<request::Login>(&request_body)
                .send()
                .await?
                .json::<response::TFIBase<response::TokenPayload>>()
                .await;
    
            token
        } 
    }


    #[cfg(test)]
    mod tests {
        use super::{*};

        #[tokio::test]
        async fn test_login() {
            let sut = TfiApi::new();

            let res = sut.login("EMAIL HERE").await.unwrap();
            println!("{}", res.payload.token);
            assert_ne!(res.payload.token, "");
            assert_eq!(res.payload.token_scheme, "Bearer");
        }
    }
}