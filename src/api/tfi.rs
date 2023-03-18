mod tfi {
    use serde::{Deserialize, Serialize};
    use reqwest::{self, header};

    use self::{response::{SocialEventPayload, SocialEventDetails}, request::{SocialEventColumn, SocialEventOrder, SocialEventSearch}};

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
            #[serde(alias = "iTotalRecords")] pub total_records: i32,
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
            pub impactful: Option<bool>,
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

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct SocialEventDetails {
            pub id: String,
            pub id_attendance: String,
            pub selection_type: i8,
            pub status: i8,
            pub start_registration: String,
            pub end_registration: String,
            pub description: String,
            pub study_program: Vec<StudyProgram>,
            pub event_category: EventCategory,
            pub scale: Scale,
            #[serde(alias="eventRequirment")] pub event_requirement: Vec<EventRequirement>,
            pub name: String,
            pub start_event_date: String,
            pub end_event_date: String,
            pub max_confirmation_date: String,
            pub student_registration_date: String,
            pub date_submitted: String,
            pub date_approved: String,
            #[serde(default = "default_value")] pub organizer: String,
            #[serde(default = "default_value")] pub role: String,
            pub impactful: Option<bool>,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct EventRequirement {
            pub name: String,
            #[serde(alias = "type")] pub req_type: String,
            pub input: String,
            pub description: String,
            pub min: f32,
            pub max: f32,
            pub ext: Option<String>,
            pub value: StringOrRequirementValueVec,
        }

        #[derive(Debug, Deserialize)]
        #[serde(untagged)]
        pub enum StringOrRequirementValueVec {
            String(String),
            Vec(Vec<RequirementValue>),
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct RequirementValue {
            pub id: String,
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

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct SocialEvent {
            pub columns: Vec<SocialEventColumn>,
            pub draw: i16,
            pub length: i8,
            pub order: Vec<SocialEventOrder>,
            pub search: SocialEventSearch,
            pub start: i16,
        }

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct SocialEventColumn {
            pub data: String,
            pub name: String,
            pub orderable: bool,
            pub searchable: bool,
        }

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct SocialEventOrder {
            pub column: String,
            pub dir: String,
        }

        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct SocialEventSearch {
            pub regex: bool,
            pub value: String,
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

        pub async fn get_social_events(&self, page: i16, length: i8, auth: &str) -> Result<response::TFIBase<SocialEventPayload>, reqwest::Error> {
            let social_event_column = vec![
                SocialEventColumn { data: "id".to_string(), name: "id".to_string(), orderable: true, searchable: true },
                SocialEventColumn { data: "name".to_string(), name: "name".to_string(), orderable: true, searchable: true }
            ];

            let social_event_order = vec![SocialEventOrder { column: "endRegistration".to_string(), dir: "desc".to_string() }];
            let social_event_search = SocialEventSearch { regex: false, value: "".to_string() };

            let request_body = request::SocialEvent {
                columns: social_event_column,
                draw: 0,
                length,
                order: social_event_order,
                search: social_event_search,
                start: 10 * (page - 1),
            };

            let res = self.client
                .post("https://tfiapi.apps.binus.ac.id/api/social-event/get-social-event-student")
                .query(&[("culture", "id-ID")])
                .bearer_auth(auth)
                .json(&request_body)
                .send()
                .await?
                .json::<response::TFIBase<SocialEventPayload>>()
                .await;

            res
        } 

        pub async fn get_social_event_details(&self, number: i16, auth: &str) -> Result<response::TFIBase<response::SocialEventDetails>, reqwest::Error> {
            let social_event_column = vec![
                SocialEventColumn { data: "id".to_string(), name: "id".to_string(), orderable: true, searchable: true },
                SocialEventColumn { data: "name".to_string(), name: "name".to_string(), orderable: true, searchable: true }
            ];

            let social_event_order = vec![SocialEventOrder { column: "endRegistration".to_string(), dir: "desc".to_string() }];
            let social_event_search = SocialEventSearch { regex: false, value: "".to_string() };

            let request_body = request::SocialEvent {
                columns: social_event_column,
                draw: 0,
                length: 1,
                order: social_event_order,
                search: social_event_search,
                start: number - 1,
            };

            let res = self.client
                .post("https://tfiapi.apps.binus.ac.id/api/social-event/get-social-event-student")
                .query(&[("culture", "id-ID")])
                .bearer_auth(auth)
                .json(&request_body)
                .send()
                .await?
                .json::<response::TFIBase<SocialEventPayload>>()
                .await.unwrap();
        
            let details = self.client
                .get("https://tfiapi.apps.binus.ac.id/api/social-event/get-social-event-student-by-id")
                .query(&[("id", res.payload.data[0].id.as_str()), ("culture", "id-ID")])
                .bearer_auth(auth)
                .send()
                .await?
                .json::<response::TFIBase<SocialEventDetails>>()
                .await;

            details
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const TOKEN: &str = "eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJodHRwOi8vc2NoZW1hcy54bWxzb2FwLm9yZy93cy8yMDA1LzA1L2lkZW50aXR5L2NsYWltcy9uYW1laWRlbnRpZmllciI6IkJOMTI0MDI4ODgzIiwiaHR0cDovL3NjaGVtYXMueG1sc29hcC5vcmcvd3MvMjAwNS8wNS9pZGVudGl0eS9jbGFpbXMvZW1haWxhZGRyZXNzIjoiYnJ5YW4uZWdiZXJ0QGJpbnVzLmFjLmlkIiwibmJmIjoxNjc5MTA3MTcyLCJleHAiOjE2NzkzNjYzNzJ9.WwFQOxB7zvp2TlKrjzeSef4NwW9k6xRPSp8t0yPa7F51TXg6zXViwry4TuWAiTS9Vz8LdhPwDuFdI2ZlgvoWHg";

        #[tokio::test]
        async fn test_login() {
            let sut = TfiApi::new();

            let res = sut.login("bryan.egbert@binus.ac.id").await.unwrap();
            println!("{}", res.payload.token);
            assert_eq!(res.status_code, 200);
            assert_ne!(res.payload.token, "");
            assert_eq!(res.payload.token_scheme, "Bearer");
        }

        #[tokio::test]
        async fn test_get_social_events() {
            let sut = TfiApi::new();

            let res = sut.get_social_events(1, 10, TOKEN).await.unwrap();
            assert_eq!(res.status_code, 200);
            assert_eq!(res.payload.data.len(), 10, "{:?}", res);
        }

        #[tokio::test]
        async fn  test_get_social_event_details() {
            let sut = TfiApi::new();

            let res = sut.get_social_event_details(1, TOKEN).await.unwrap();

            assert_eq!(res.status_code, 200);
            assert_ne!(res.payload.id, "");
        }
    }
}