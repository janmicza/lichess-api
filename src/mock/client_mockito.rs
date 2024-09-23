use crate::client::LichessApi;
use crate::error::{Error, Result};
use crate::model::games::*;
use regex::Regex;
use std::sync::LazyLock;

pub struct Mock;

impl LichessApi<Mock> {
    pub async fn export_one_game(
        &self,
        request: impl Into<export::one::GetRequest>,
    ) -> Result<GameJson> {
        self.export_one_game_sync(request)
    }

    fn export_one_game_sync(
        &self,
        request: impl Into<export::one::GetRequest>,
    ) -> Result<GameJson> {
        let game_id = Self::extract_game_id(request.into().path.as_str()).ok_or(
            Error::Response("invalid path, expected /game/export/{game_id}".to_string()),
        )?;

        Ok(self.game_by_id(game_id.as_str())?)
    }

    fn game_by_id(&self, game_id: &str) -> Result<GameJson> {
        static GAME_0J36WF0D: LazyLock<GameJson> =
            LazyLock::new(|| serde_json::from_str(include_str!("./data/0j36wf0d.json")).unwrap());
        static GAME_QAPYIPOM: LazyLock<GameJson> =
            LazyLock::new(|| serde_json::from_str(include_str!("./data/qapyipom.json")).unwrap());

        match game_id.to_lowercase().as_str() {
            "0j36wf0d" => Ok(GAME_0J36WF0D.clone()),
            "qapyipom" => Ok(GAME_QAPYIPOM.clone()),
            _ => Err(Error::Response(
                "supported games are only: 0j36wf0d, qapyipom".to_string(),
            )),
        }
    }

    fn extract_game_id(path: &str) -> Option<String> {
        static EXTRACT_GAME_ID_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"^/game/export/(?P<game_id>[^/]+)$").unwrap());

        if let Some(captures) = EXTRACT_GAME_ID_REGEX.captures(path) {
            Some(captures["game_id"].to_string())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::games::export::one::{GetQuery, GetRequest};

    #[test]
    fn test_export_one_game_sync_valid_game_id() {
        let api = LichessApi::new(Mock, None);

        let game = api.export_one_game_sync(GetRequest::new("0J36WF0D", GetQuery::default()));
        assert!(game.is_ok());
        assert_eq!(game.unwrap().id, "0J36WF0D");

        let game = api.export_one_game_sync(GetRequest::new("qAPyiPom", GetQuery::default()));
        assert!(game.is_ok());
        assert_eq!(game.unwrap().id, "qAPyiPom");
    }

    #[test]
    fn test_export_one_game_sync_invalid_game_id() {
        let api = LichessApi::new(Mock, None);
        let game = api.export_one_game_sync(GetRequest::new("non-existent", GetQuery::default()));
        assert!(matches!(game, Err(Error::Response(_))));
        assert_eq!(
            game.err().unwrap().to_string(),
            "response error: supported games are only: 0j36wf0d, qapyipom"
        );
    }
}
