pub mod protocol;
pub use protocol::*;

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn parse_wallet_msg() {
    let data = "{\"type\":\"chat\",\"source\":\"remote\",\"msg\":{\"id\":119,\"conversation_id\":\"000032a6ec0a894bd95fe8aa4ff721251aa3b3d267ae123b6a7bf7b735ec0dd4\",\"channel\":{\"name\":\"crypticio\",\"public\":false,\"members_type\":\"team\",\"topic_type\":\"chat\",\"topic_name\":\"bot-test\"},\"sender\":{\"uid\":\"4c230ae8d2f922dc2ccc1d2f94890700\",\"username\":\"marcopolo\",\"device_id\":\"c56478e637d5bccffaa11ba8fbb15118\",\"device_name\":\"MKB_LITHOP_2\"},\"sent_at\":1569372355,\"sent_at_ms\":1569372355569,\"content\":{\"type\":\"text\",\"text\":{\"body\":\"+0.0001xlm@marco \",\"payments\":[{\"username\":\"marco\",\"paymentText\":\"+0.0001XLM@marco\",\"result\":{\"resultTyp\":0,\"sent\":\"1906082074a29e9496efcfc298536b4cf4bd9ce49a514c1918895b77ec81e255\"}}],\"userMentions\":[{\"text\":\"\",\"uid\":\"xkmyj/qxlYbbKqnsdWzOAA==\"}],\"teamMentions\":null}},\"prev\":null,\"unread\":false,\"at_mention_usernames\":[\"marco\"],\"channel_mention\":\"none\"},\"pagination\":{\"next\":\"76\",\"previous\":\"76\",\"num\":1,\"last\":false}}\n";
    let _v: api::MsgNotification = serde_json::from_str(data).unwrap();
  }
}
