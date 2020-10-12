use std::fs::File;
use std::io::prelude::*;

use askama::Template;

#[derive(Template)]
#[template(path = "linzklar.html")]
struct LinzklarTemplate<'a> {
    linzi: &'a str,
    toc: &'a str,
    content: &'a str,
}

use big_s::S;

enum Foo {
    L(String),
    C(String, Vec<Foo>, String),
}

impl Foo {
    pub fn strs(&self) -> Vec<String> {
        match self {
            Foo::L(s) => vec![s.clone()],
            Foo::C(s, t, u) => {
                let mut ans = vec![s.clone()];
                for a in t {
                    let mut k: Vec<_> = a.strs().into_iter().map(|b| format!("  {}", b)).collect();
                    ans.append(&mut k);
                }
                ans.push(u.clone());
                ans
            }
        }
    }
}

impl std::fmt::Display for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.strs().join("\n"))
    }
}

fn generate_toc(toc: Vec<(&str, Vec<&str>)>) -> String {
    let mut global_ind = 0;
    Foo::C(
        S(r##"<ol class="goog-toc">"##),
        toc.into_iter()
            .enumerate()
            .map(|(sec_num_minus_1, t)| {
                Foo::C(
                    format!(
                        r##"<li class="goog-toc"><a href="#TOC--{}"><strong>{} </strong>{}</a>"##,
                        if global_ind == 0 {
                            S("")
                        } else {
                            format!("{}", global_ind)
                        },
                        sec_num_minus_1 + 1,
                        t.0
                    ),
                    vec![Foo::C(
                        S(r##"<ol class="goog-toc">"##),
                        {
                            let mut v = vec![];
                            global_ind += 1;
                            let mut subsec_num = 1;
                            for a in t.1 {
                                v.push(Foo::L(format!(
                                    r##"<li class="goog-toc"><a href="#TOC--{}"><strong>{}.{}
          </strong>{}</a></li>"##,
                                    global_ind,
                                    sec_num_minus_1 + 1,
                                    subsec_num,
                                    a
                                )));
                                global_ind += 1;
                                subsec_num += 1;
                            }

                            v
                        },
                        S(r##"</ol>"##),
                    )],
                    S(r##"</li>"##),
                )
            })
            .collect(),
        S("</ol>"),
    )
    .to_string()
}

fn content() -> String {
    r##"<div>
  <div style="display:block;text-align:left">
      <div style="display:block;text-align:left">
              <div style="display:block;text-align:left"><img src="linzi/在.png"
                  border="0"></div>
          <div style="display:block;text-align:left">総画：4</div>
          <div style="display:block;text-align:left">筆順：丶ノ一一</div>
          <h3 style="display:block;text-align:left"><a name="TOC--1"></a>字源</h3>
          <div>
            <ul>
              <li>象形指事。「<a href="処%20-%20燐字海.html">処</a>」を強調したもの。
              </li>
            </ul>
            <div>
              <div style="font-size:13.3333px">
                <h3><a name="TOC--2"></a>キャスカ・ファルザーの字源</h3>
                <div style="font-size:13.3333px">
                  <ul></ul>
                </div>
              </div>
              <div style="font-size:13.3333px">
                <ul>
                  <li>
                    呪術において使われる祭壇に乗せられる器を表す。器に供え物を置くという行為が、文化的な観点で強く「存在」を表したために、一般的な存在の意に転義した。
                  </li>
                </ul>
              </div>
            </div>
            <div>
              <div style="display:block;text-align:left">
                <div style="display:block;text-align:left"></div>
                <div style="display:block;text-align:left"><img
                    src="grau_prua_yr/在.png" width="200" height="91" border="0">
                </div>
              </div>
            </div>
          </div>
          <div></div>
          <h3><a name="TOC--3"></a>意義</h3>
          <div>
            <ol>
              <li>在る。</li>
            </ol>
          </div>
          <div><br></div>
          <h2><a name="TOC--4"></a><a
              href="https://sites.google.com/site/syxobo/raneme-zu-yu">ラネーメ祖語</a>
          </h2>
          <div>
            <h3><a name="TOC--5"></a>
              <hr>発音</h3>
          </div>
          <div>aimq</div>
          <h3><a name="TOC--6"></a>名詞</h3>
          <div>存在。</div>
          <h3><a name="TOC--7"></a>述詞</h3>
          <div>在る。～している。</div>
          <h2><a name="TOC--8"></a><a
              href="https://sites.google.com/site/riparaincangku/yuesureone-ren-gong-shi-jie-she-ding/pai-sheng-yu-fang-yan/lkurftlessd-air">アイル語</a>
          </h2>
          <div>
            <hr>
          </div>
        
        <h3 style="display:block;text-align:left"><a name="TOC--9"></a>発音</h3>
        <div>aima</div>
        <h3><a name="TOC--10"></a>動詞</h3>
        <div>在る。</div>
        <h2><a name="TOC--11"></a><a
            href="https://sites.google.com/site/syxobo/paigu-yu">パイグ語</a></h2>
        <div>
          <hr>
          <h3><a name="TOC--12"></a>発音</h3>
        </div>
        <div>
          <ul>
            <li><span
                style="font-size:10pt;background-color:transparent">標準パイグ語：aim2</span>
            </li>
            <li><span
                style="font-size:10pt;background-color:transparent">アイツォ語：aim2</span>
            </li>
            <li><span
                style="font-size:10pt;background-color:transparent">古音：raim</span>
            </li>
            <li><span
                style="font-size:10pt;background-color:transparent">韻図音：冠在素</span>
            </li>
          </ul>
        </div>
        <div>
          <h3><a name="TOC--13"></a>名詞</h3>
        </div>
        <div>存在。</div>
        <h3><a name="TOC--14"></a>動詞</h3>
        <div>在る。</div>
        <h3><a name="TOC--15"></a>定詞</h3>
        <div>～している。</div>
        <h3><a name="TOC--16"></a>叫詞</h3>
        <div>はい。</div>
        <div><br></div>
        <h2><a name="TOC--17"></a><a
            href="https://sites.google.com/site/syxobo/takan">タカン語</a></h2>
        <div>
          <hr>
        </div>
      </div>
      <div style="display:block;text-align:left">
        <div style="font-size:13.3333px">
          <h3><a name="TOC--18"></a>発音</h3>
        </div>
        <div>
          <ul>
            <li><span style="background-color:transparent">
                <font size="2">皇音：えま、え-む</font>
              </span></li>
            <li>
              <font size="2"><span
                  style="background-color:transparent">牌音</span><span
                  style="background-color:transparent">　古音：アイ　</span><span
                  style="background-color:transparent">新音：エン</span></font>
            </li>
          </ul>
        </div>
        <div style="font-size:13.3333px">
          <h3><a name="TOC--19"></a>名詞</h3>
        </div>
        <div style="font-size:13.3333px">（えま<span
            style="font-size:small;background-color:transparent">）</span><span
            style="font-size:13.3333px;background-color:transparent">存在。</span>
        </div>
        <h3><a name="TOC--20"></a>
          <font size="3">動詞</font>
        </h3>
        <div>
          <font size="2">（え-む）ある。</font><span
            style="font-size:13.3333px;background-color:transparent">～している。</span>
        </div>
        <div>
          <div style="font-size:13.3333px">
            <h2><a name="TOC--21"></a><a
                href="https://sites.google.com/site/riparaincangku/yuesureone-ren-gong-shi-jie-she-ding/pai-sheng-yu-fang-yan/lkurftlessd-air/etz">エッツィア語</a>
            </h2>
            <div>
              <hr>
            </div>
          </div>
          <div>
            <div style="font-size:13.3333px">
              <h3><a name="TOC--22"></a>発音</h3>
            </div>
            <div>
              <ul>
                <li><span style="background-color:transparent">
                    <font size="2">光音：あいま</font>
                  </span></li>
                <li><span style="background-color:transparent">
                    <font size="2">皇音：え、えむ</font>
                  </span></li>
                <li>
                  <font size="2"><span
                      style="background-color:transparent">牌音　</span><span
                      style="background-color:transparent">古音：ラン　</span><span
                      style="background-color:transparent">現音：アン</span></font>
                </li>
              </ul>
            </div>
            <div style="font-size:13.3333px">
              <h3><a name="TOC--23"></a>名詞</h3>
            </div>
            <div>存在、あること</div>
          </div>
        </div>
        <div>
          <h3><a name="TOC--24"></a>動詞</h3>
        </div>
        <div>（えま、アン）在る、存在する　（あいま）行う、実行する</div>
        <div style="font-size:13.3333px">
          <h2><a name="TOC--25"></a><a
              href="http://jurliyuuri.github.io/bhaataan/grammar.html">バート語</a>
          </h2>
          <div>
            <hr>
          </div>
        </div>
      </div>
      <div>
        <h3><a name="TOC--26"></a>
          <font size="3">発音</font>
        </h3>
        <div>hemúl, hem</div>
      </div>
      <h3><a name="TOC--27"></a>動詞</h3>
      <div>(hemúl) ある。</div>
      <div>
        <h3><a name="TOC--28"></a>無変化動詞</h3>
      </div>
      <div>(hem) 完了の無変化動詞。〜である。</div>
      <div><br></div>
    
    <div style="font-size:13.3333px">
      <h2><a name="TOC--29"></a><a
          href="https://sites.google.com/site/3tvalineparine/home">リパライン語</a></h2>
      <div>
        <hr>
      </div>
    </div>
    <h3><a name="TOC--30"></a>発音</h3>
    <div>
      <ol>
        <li>es e'i</li>
        <li>teles</li>
        <li>mol</li>
        <li>molo</li>
        <li>molerl</li>
      </ol>
    </div>
    <h3><a name="TOC--31"></a>名詞</h3>
    <div>在ること、存在</div>
    <div>
      <h3><a name="TOC--32"></a>動詞</h3>
    </div>
    行う、存在する（行うの文脈の場合、目的語があるならtelesで、無い場合はes e'iで訓読する。）
    <h3><a name="TOC--33"></a>熟語</h3>
    <ol>
      <li><a href="真%20-%20燐字海.html">真</a>在　xinien
        la deliume　＜本分、本来の義務＞</li>
    </ol>
  </div>
</div>"##.to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(format!("docs/{} - 燐字海.html", "在"))?;
    write!(
        file,
        "{}",
        LinzklarTemplate {
            linzi: "在",
            toc: &generate_toc(vec![
                ("燐字", vec!["字源", "キャスカ・ファルザーの字源", "意義"]),
                ("ラネーメ祖語", vec!["発音", "名詞", "述詞"]),
                ("アイル語", vec!["発音", "動詞"]),
                ("パイグ語", vec!["発音", "名詞", "動詞", "定詞", "叫詞"]),
                ("タカン語", vec!["発音", "名詞", "動詞"]),
                ("エッツィア語", vec!["発音", "名詞", "動詞"]),
                ("バート語", vec!["発音", "動詞", "無変化動詞"]),
                ("リパライン語", vec!["発音", "名詞", "動詞", "熟語"])
            ]),
            content: &content()
        }
        .render()
        .unwrap()
    )?;

    Ok(())
}
