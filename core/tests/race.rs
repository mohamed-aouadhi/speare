use speare::*;
use std::time::Duration;
use tokio::time;

struct Wait(u8);
struct Immediate(u8);
struct Get;

#[derive(Default)]
struct Bag {
    msgs: Vec<u8>,
}

impl Process for Bag {}

#[process]
impl Bag {
    #[handler]
    async fn wait(&mut self, msg: Wait, _: &Ctx<Self>) -> Result<(), ()> {
        time::sleep(Duration::from_millis(100)).await;
        self.msgs.push(msg.0);
        Ok(())
    }

    #[handler]
    async fn immediate(&mut self, msg: Immediate, _: &Ctx<Self>) -> Result<(), ()> {
        self.msgs.push(msg.0);
        Ok(())
    }

    #[handler]
    async fn get(&mut self, _: Get, _: &Ctx<Self>) -> Result<Vec<u8>, ()> {
        Ok(self.msgs.clone())
    }
}

#[tokio::test]
async fn no_data_races() {
    // Arrange
    let node = Node::default();
    let bag = node.spawn(Bag::default()).await;

    // Act
    node.tell(&bag, Wait(0)).await;
    node.tell(&bag, Immediate(1)).await;
    let actual = node.ask(&bag, Get).await.unwrap_or_else(|_| vec![]);

    // Assert
    let expected = vec![0, 1];

    assert_eq!(actual, expected);
}
