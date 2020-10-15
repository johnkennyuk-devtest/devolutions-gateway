use crate::{
    get_uuid_in_path,
    utils::{RequestHelper, ResponseHelper},
    Error, JET_HEADER_HOST, JET_HEADER_VERSION,
};
use http::StatusCode;
use std::io;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct JetTestReq {
    pub version: u32,
    pub host: String,
    pub association: Uuid,
    pub candidate: Uuid,
}

impl JetTestReq {
    pub fn to_payload(&self, mut stream: impl io::Write) -> Result<(), Error> {
        stream.write_fmt(format_args!(
            "GET /jet/test/{}/{} HTTP/1.1\r\n",
            &self.association.to_string(),
            &self.candidate.to_string()
        ))?;
        stream.write_fmt(format_args!("Host: {}\r\n", &self.host))?;
        stream.write_fmt(format_args!("Connection: Close\r\n"))?;
        stream.write_fmt(format_args!("Jet-Version: {}\r\n", &self.version.to_string()))?;
        stream.write_fmt(format_args!("\r\n"))?;
        Ok(())
    }

    pub fn from_request(request: &httparse::Request) -> Result<Self, Error> {
        if request.is_get_method() {
            let version_opt = request
                .get_header_value(JET_HEADER_VERSION)
                .and_then(|version| version.parse::<u32>().ok());
            let host_opt = request.get_header_value(JET_HEADER_HOST);

            if let (Some(version), Some(host)) = (version_opt, host_opt) {
                if let Some(path) = request.path {
                    if path.starts_with("/jet/test") {
                        if let (Some(association_id), Some(candidate_id)) =
                            (get_uuid_in_path(path, 2), get_uuid_in_path(path, 3))
                        {
                            return Ok(JetTestReq {
                                version,
                                host: host.to_string(),
                                association: association_id,
                                candidate: candidate_id,
                            });
                        }
                    }
                }
            }
        }
        Err(format!("Invalid test request: {:?}", request).into())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct JetTestRsp {
    pub status_code: StatusCode,
    pub version: u32,
}

impl JetTestRsp {
    pub fn to_payload(&self, mut stream: impl io::Write) -> Result<(), Error> {
        stream.write_fmt(format_args!(
            "HTTP/1.1 {} {}\r\n",
            &self.status_code,
            self.status_code.as_str()
        ))?;
        stream.write_fmt(format_args!(
            "{}: {}\r\n",
            JET_HEADER_VERSION,
            &self.version.to_string()
        ))?;
        stream.write_fmt(format_args!("\r\n"))?;
        Ok(())
    }

    pub fn from_response(response: &httparse::Response) -> Result<Self, Error> {
        if let Some(status_code) = response.code.and_then(|code| StatusCode::from_u16(code).ok()) {
            let version_opt = response
                .get_header_value(JET_HEADER_VERSION)
                .and_then(|version| version.parse::<u32>().ok());

            match version_opt {
                Some(2) => {
                    return Ok(JetTestRsp {
                        status_code,
                        version: 2,
                    });
                }
                _ => {}
            }
        }

        Err(format!("Invalid test response: {:?}", response).into())
    }
}