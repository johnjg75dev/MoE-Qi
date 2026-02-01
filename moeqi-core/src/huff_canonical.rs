#[derive(Default)]
pub struct Node {
    pub sym: Option<i16>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub fn build_tree(symbols: &[i16], lengths: &[u8]) -> Result<Node, ()> {
    if symbols.len() != lengths.len() { return Err(()); }
    let mut pairs: Vec<(u8, i16)> = symbols.iter().copied().zip(lengths.iter().copied())
        .map(|(s,l)| (l,s)).collect();
    pairs.sort_by_key(|(l,s)| (*l, *s));

    if pairs.is_empty() { return Err(()); }

    let mut root = Node::default();
    let mut code: u32 = 0;
    let mut prev_len = pairs[0].0 as u32;

    for (idx, (len, sym)) in pairs.into_iter().enumerate() {
        let len = len as u32;
        if idx == 0 {
            code = 0;
            prev_len = len;
        } else {
            code = (code + 1) << (len - prev_len);
            prev_len = len;
        }
        // insert bits MSB->LSB
        let mut node = &mut root;
        for i in (0..len).rev() {
            let bit = (code >> i) & 1;
            if bit == 0 {
                if node.left.is_none() { node.left = Some(Box::new(Node::default())); }
                node = node.left.as_deref_mut().unwrap();
            } else {
                if node.right.is_none() { node.right = Some(Box::new(Node::default())); }
                node = node.right.as_deref_mut().unwrap();
            }
        }
        node.sym = Some(sym);
    }

    Ok(root)
}
