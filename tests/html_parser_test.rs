use sauron::html::lookup::match_tag;
use sauron::vdom::Node;
use sauron_html_parser::parse_html;

#[test]
fn should_match_tags() {
    assert_eq!(Some("div"), match_tag(&String::from("div")));
    assert_eq!(Some("svg"), match_tag(&String::from("svg")));
    assert_eq!(
        Some("color-profile"),
        match_tag(&String::from("color-profile"))
    );
}

// #[test]
// fn test_html_child() {
//     let html = r#"<article class="side-to-side">
//     <div>
//         This is div content1
//     </div>
//     <footer>
//         This is footer
//     </footer>
// </article>"#;
//     let expected = "<article class=\"side-to-side\"><div>\n        This is div content1\n    </div><footer>\n        This is footer\n    </footer></article>";
 
//     let node: Node<()> = parse_html(html).ok().flatten().expect("must parse");
//     println!("node: {:#?}", node);
//     println!("render: {}", node.render_to_string());
//     assert_eq!(expected, node.render_to_string());
// }

#[test]
fn test_node_list() {
    let html = r#"<!doctype html>
    <html>
        <body>This is body</body>
    </html>"#;
    let expected = "<html><body>This is body</body></html>";
    let node: Node<()> = parse_html(html).ok().flatten().expect("must parse");
    println!("node: {:#?}", node);
    println!("render: {}", node.render_to_string());
    assert_eq!(expected, node.render_to_string());
}

#[test]
fn test_inline_style() {
    let html = r#"<div id="there"><img src="posts/libnix/Nix_snowflake_windows.svg" class="noFancy" style="float: right;" width="200px"/></div>"#;
    let expected = r#"<div id="there"><img src="posts/libnix/Nix_snowflake_windows.svg" class="noFancy" style="float:right;" width="200px"/></div>"#;
    let node: Node<()> = parse_html(html).ok().flatten().expect("must parse");
    println!("node: {:#?}", node);
    println!("render: {}", node.render_to_string());
    assert_eq!(expected, node.render_to_string());
}

#[test]
fn test_pre_code() {
    let html =  
r#"<div><p> test </p>
<pre><code><p>foo1</p>
  <p>foo2</p><p>foo3</p>
  3</code></pre></div>"#;
let expected = 
r#"<div><p> test </p>
<!--separator--><pre><code><p>foo1</p>
  <p>foo2</p><p>foo3</p>
  3</code></pre></div>"#;

    let node: Node<()> = parse_html(html).ok().flatten().expect("must parse");
    //println!("-------------------------------");
    //println!("node: \n{:#?}", node);
    //println!("-------------------------------");
    //println!("html: \n{}", html);
    //println!("-------------------------------");
    //println!("render_to_string: \n{}", node.render_to_string());
    //println!("-------------------------------");
    //println!("render_to_string_pretty: \n{}", node.render_to_string_pretty());
    //println!("-------------------------------");
    assert_eq!(expected, node.render_to_string());
    //assert_eq!(1,2);
}

#[test]
fn test_pre_code3() {
    let html = r#"<div><p> test </p><pre><code>
0
  1
  2
3
</code></pre>
</div>"#;
let expected = r#"<div><p> test </p><pre><code>
0
  1
  2
3
</code></pre><!--separator-->
</div>"#;

    let node: Node<()> = parse_html(html).ok().flatten().expect("must parse");
    //println!("node: {:#?}", node);
    println!("html: {}", html);
    println!("render: {}", node.render_to_string());
    assert_eq!(expected, node.render_to_string());
}

#[test]
fn test_pre_code3_paragraphs_mix() {
  let html = r#"<div><p> test </p><pre><code>
  0
  <p>1</p>
  2
<p>3</p>
  4
</code></pre>
</div>"#;
  let expected = r#"<div><p> test </p><pre><code>
  0
  <p>1</p>
  2
<p>3</p>
  4
</code></pre><!--separator-->
</div>"#;

    let node: Node<()> = parse_html(html).ok().flatten().expect("must parse");
    //println!("node: {:#?}", node);
    println!("html: {}", html);
    println!("render: {}", node.render_to_string());
    assert_eq!(expected, node.render_to_string());
    // right "<div><p> test </p><pre><code>\n  0\n  <p>1</p><p>2</p><p>3</p></code></pre></div>"
}

#[test]
fn test_pre_code_2() {
    let html = r#"<pre><code>
<span>asdf</span>
  <span>asdf</span>
  <span>asdf</span>
</code></pre>"#;
let expected = r#"<pre><code>
<span>asdf</span>
  <span>asdf</span>
  <span>asdf</span>
</code></pre>"#;

    let node: Node<()> = parse_html(html).ok().flatten().expect("must parse");
    //println!("node: {:#?}", node);
    println!("html: {}", html);
    println!("render: {}", node.render_to_string());
    // right is: "<pre><code><span>asdf</span><span>asdf</span><span>asdf</span></code></pre>"
    assert_eq!(expected, node.render_to_string());
}
