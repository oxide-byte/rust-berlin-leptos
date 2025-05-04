use crate::model::MeetupUrl;
use serde_json::Error;
use std::fs;
use std::io::BufReader;

pub fn import_data() -> Vec<MeetupUrl> {
    let file = fs::File::open("../data/data.json")
        .expect("Error opening the file");

    let reader = BufReader::new(file);

    let json: Result<Vec<MeetupUrl>, Error> = serde_json::from_reader(reader);
    if json.is_ok() {
        json.unwrap()
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::MeetupUrl;
    use crate::service::import_data;
    use serde_json::Error;

    #[test]
    fn test_object() {
        let data = r#"
          {
            "uri_uuid": "c750fdb6b6b4f69d9e1d293775e917da5257be09260174e25f0fb89e4cba8d0f",
            "url": "https://2024.rustnl.org/live/",
            "scheme": "https",
            "host": "2024.rustnl.org",
            "path": "/live/",
            "live_status": "1",
            "title": "🔴 Live recordings - RustNL 2024",
            "auto_descr": "RustNL 2024, The Netherlands - May 7 & 8 2024",
            "man_descr": "",
            "crea_user": "api",
            "crea_time": "2024-08-29T17:48:10.384049834+02:00",
            "modi_user": "api",
            "modi_time": "2024-08-29T17:48:10.384049834+02:00"
          }
        "#;

        let result: Result<MeetupUrl, Error> = serde_json::from_str(data);
        assert_eq!(result.is_ok(), true);
        let url = result.unwrap();
        println!("{:?}", url);
    }

    #[test]
    fn test_import() {
        let data = import_data();
        assert_eq!(data.is_empty(), false);
    }
}