use crate::{
    error::SdkError,
    mp::event::{xmlutil::get_text_from_root, ReceivedMessageParser},
    SdkResult,
};

use super::{event_scan::ScanEvent, EventMessage};

pub struct SubScribeEvent;

impl ReceivedMessageParser for SubScribeEvent {
    type ReceivedMessage = EventMessage;

    fn from_xml(node: &roxmltree::Node) -> SdkResult<Self::ReceivedMessage> {
        let ekn = node.descendants().find(|n| n.has_tag_name("EventKey"));
        let event = match ekn {
            Some(n) => {
                let event_key = n.text().unwrap_or("");
                if event_key.is_empty() {
                    return EventMessage::Subscribe;
                }

                let ticket = get_text_from_root(node, "Ticket")?;
                EventMessage::SubscribeScan(ScanEvent {
                    event_key: event_key.to_string(),
                    ticket: ticket.to_string(),
                })
            }
            None => EventMessage::Subscribe,
        };
        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mp::event::EventMessage;
    use crate::SdkResult;
    #[test]
    pub fn parse() -> SdkResult<()> {
        use roxmltree::Document;
        let s = "<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[FromUser]]></FromUserName>
    <CreateTime>123456789</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[subscribe]]></Event>
    <EventKey><![CDATA[qrscene_123123]]></EventKey>
    <Ticket><![CDATA[TICKET]]></Ticket>
</xml>";
        let node = Document::parse(&s)?;
        let msg = SubScribeEvent::from_xml(&node.root())?;

        matches!(msg, EventMessage::SubscribeScan(_));
        Ok(())
    }
}
