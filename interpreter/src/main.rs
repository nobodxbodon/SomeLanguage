extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::Pair;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "some_language.pest"]
pub struct SomeLanguageParser;



fn pack_vars(block:Pair<Rule>)->HashMap<String,Option<isize>>{
    let mut global_map=HashMap::new();
    for content in block.into_inner(){
        for record in content.into_inner(){
            match record.as_rule(){
                Rule::askUnknown=>{
                    let mut token_iter =record.into_inner();
                    let name=token_iter.next().unwrap().as_str().to_string();
                    global_map.insert(name,None);
                },
                Rule::statement=>{
                    let mut token_iter =record.into_inner();
                    let name=token_iter.next().unwrap().as_str().to_string();
                    token_iter.next();
                    let value=token_iter.next().unwrap().as_str().to_string().parse::<isize>().unwrap();
                    global_map.insert(name,Some(value));
                },
                Rule::relation=>{
                    //todo! 暂不实现
                },
                _=>{
                    panic!("How can you reach here?");
                }
            }
        }
    }
    return global_map;
}


fn main() {
    let text="摄氏度是38，华氏度是多少";
    let mut content=SomeLanguageParser::parse(Rule::block,text).unwrap_or_else(|e| panic!("{}", e));
    let m=pack_vars(content.next().unwrap());
    println!("{:?}",m);
    //输出：{"摄氏度": Some(38), "华氏度": None}
}

