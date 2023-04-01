use super::*;

#[test]
fn parse_empty() {
    let nodes = parse_tree(Vec::new());
    assert_eq!(nodes.len(), 0);
}