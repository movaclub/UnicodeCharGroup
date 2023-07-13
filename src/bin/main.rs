
use unicode_char_group::UnicodeCharGroup::*;


fn main() {
    
    let str = String::from("王何必曰利？亦有仁義而已矣。Being wise and good, they have pleasure in these things. If they are not wise and good, though they have these things, they do not find pleasure.");

    println!("\n\n\tTotal words: {}\n\n", str.chars().count());

    let res = stats(&str);

    for (k,v) in res.iter() {
        println!("\t{k}");
        for stat in v.iter() {
            println!("\t\t{}", stat.block);
            println!("\t\t\tChars: {}", stat.counter);
            println!("\t\t\tPercentage: {}%\n", stat.proportion);
        }
        println!()
    }

    
}
