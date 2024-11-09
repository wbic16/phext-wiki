use libphext::phext;

pub struct PhextWikiNode {
    pub detail: phext::PositionedScroll
}
impl PhextWikiNode {
    fn name(&self) -> String {
        phext::create_summary(self.detail.scroll.as_str())
    }

    fn new() -> PhextWikiNode{
        PhextWikiNode { detail: phext::PositionedScroll::new() }
    }
}

fn main() {
    let mut test = PhextWikiNode::new();
    test.detail.coord = phext::to_coordinate("1.4.4/5.2.1/6.6.4");
    test.detail.scroll = "Hello Phext-Wiki\nSecond line".to_string();

    println!("{}: {}", test.detail.coord, test.name());
}