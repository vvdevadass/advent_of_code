use regex::Regex;

fn mult(line: &str) -> i64 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let re_num = Regex::new(r"\d{1,3}").unwrap();
    let mut res = 0;
    let subs: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
    for s in subs {
        let mut mul = 1;
        for num in re_num.find_iter(s).map(|m| m.as_str().parse::<i64>().unwrap()) {
            mul *= num;
        }
        if mul > 1 {
            res+=mul;
        }
    }
    res
}

pub fn part_a(input: &str) -> i64 {
    let mut res = 0;
    for line in input.trim().split('\n') {
        res += mult(line);
    }
    res
}

pub fn part_b(input: &str) -> i64 {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let mut res = 0;
    let mut is_enabled = true;

    for line in input.trim().split('\n') {
        let mat: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        for s in mat {
            match s {
                "don't()" => { is_enabled = false; }
                "do()" => { is_enabled = true; }
                _ => {
                    if is_enabled {
                        res+=mult(s);
                    }
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("sample.txt")), 161);
        assert_eq!(super::part_a(include_str!("input.txt")), 171183089);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("sample.txt")), 48);
        // 53984975 < x < 69346806
        assert_eq!(super::part_b(include_str!("input.txt")), 63866497);
        // let s = "};//how():mul(422,702)'how()'from()-&when(551,888)from()#mul(694,437)who()<,/ ~)@&mul(776,511)@]%'({*select()mul(314,525)}}#]{!,]-mul(320,780)-who()mul(658,818)*%&>$who()(;:mul(387,867)@^where()-mul(901,72)mul(834,66)-what()how(){when()from()~why(872,51)what()mul(140,14)](who()?@}mul(85,947)(don't()/ how()*$[mul(797,644)mul(317,180)()where(480,157)how()when()^[ @what()mul(878,580)+;}>,~}&,(mul(623,287)mul(673,493)what()^?)-]@;mul(816,370))@+!><]mul(5,500)from(54,216)% :mul(815,27)(+<;mul(285,821)mul(71,366)mul(673,217)({('-)/>!mul(823,776),')%mul(738,626){ ]mul(805,547)mul(415,54)^ !![from()(mul(66,234)when(811,231)where()/%<{)mul(934,395) }~select(673,484)<-%from()?why()mul(522,155)}}>{~*%*mul(728,428)#>,,who()% don't()from()@/$@mul(85,666)#mul(248,114)<^from()@where()#who()@]mul(828,164)~]mul(629,878)&*'don't()& (% [-]mul(559,530)^';;>%'-mul(691,844)select()(where()#{where()>$?mul(430,663) (')/?where()~mul(490,843)mul(732,371)what()mul(941,911)&>,#,what()<what(668,239),mul(852,922)-@select()@select()mul(164,825) #!from()mul(438,366)&when(337,694)>: -];where(533,294)mul(432,502)where()mul(260,676)*mul(523,624)$:from()%@mul(826,610)when()~]']mul(233,752)*#)select()when())mul(566,872))?#/}what()<mul(971,886)(]%mul(534,431),%mul(96,804)] -+&:mul(316,380)who()&how()/mul(235,477)/mul(673,227)what()$]+why()what()$;why()don't() what()@)@:mul(936,387)*mul(541,493)(from(134,594)$@# &what()mul(815,777)~don't()~/)from()select()where()what()who(813,632)![mul(406,400)mul(500,187)~,(,;<mul(417,43)what()why(962,497)@@where()how()[don't()-!@{,when()?<mul(631,173)[/{?]'~)&mul(52,882)~&%why()#^mul(355,511)@who())?-mul(290,40) /mul(904,348)what()mul(64,672)what()#<</([/what()#mul(919,764)where()-&&&>where(499,38)(@-mul(715}#%!mul(769,30)+#^+mul(990,690)where(879,83))<*when()'mul(215,273)what()why()]why()^what()from()mul(415,131)/~mul*]mul(486,36)~]how()-]@<?mul(72@[what()[[+![mul(361,609)[what()#[,mul(588,160)][mul(20,721):where()&%%where()what()&mul(656,757)what(227,948)how()$from()why(703,434)$mul(71,245)$when()how()from()%,{( %mul(863,937]&mul(77,174)where()-?who()-mul(686,66)[select(731,328)(what()mul(3,841) #(who()%/mul(959,234)',^who(101,965)<##)(mul(678,85)/@';;where()mul(307,32)what()&/!)mul(220,161);^mul(930,751)what()mul(170,975)select()};?+select()when();)mul(36,174){!%who(),</:mul(260,293)/:)from()<who()}mul(809,643):>[;$*mul(906,502)why()when()^mul(545{;what()do())select();&mulwhere(920,781)how()@;mul(674,946)<,>what())when()^don't()mul(724,896)}; (from()'mul(224,689)'%+mul(414,951)*>::*:@#?mul(53,511)where()*how()^@<mul(86,654)~@~,{mul(375,691)[?select()where()?select()[why()[ mul(669,31)}{*select()}where()from()where()mul(95,352)>who()~select(619,591)'from(),from()[?mul(93,443)%;mul(261,326)':'}'select()#mul(360,973)select(284,94)$mul(175,634)?when(811,326)~%where(843,474)mul(765,798):mul(673,144):where()?mul(159,258)&mul(815,9)<mul(834,28)^from()what(42,833)mul(553,672)%?from();!<'where()#?mul(896,767)";
        // assert_eq!(super::part_b(s), 0);
    }
}
