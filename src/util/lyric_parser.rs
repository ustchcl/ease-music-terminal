use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub struct LyricRow {
    pub start: i32,
    pub content: String,
}


impl LyricRow {
    pub fn new(start: i32, content: String) -> Self {
        Self {
            start, content
        }
    }
}

pub fn parse_rows(content: &str) -> Vec<LyricRow> {
    let re = Regex::new(r"\[(\d{2}):(\d{2})\.(\d+)\](.*)").unwrap();
    let mut rows = vec![];
    for line in content.lines() {
        if let Some(lyric_row) = parse_row(line, &re) {
            rows.push(lyric_row);
        }
    }
    rows
}

fn parse_row(line: &str, re: &Regex) -> Option<LyricRow> {
    let captures = re.captures(line)?;
    let minute_str = captures.get(1)?.as_str();
    let seconds_str = captures.get(2)?.as_str();
    // let millseconds_str = captures.get(2)?.as_str();
    let content_str = captures.get(4)?.as_str();
    let time = parse_time(minute_str, seconds_str)?;
    Some(
        LyricRow::new(time, content_str.to_string())
    )
}

fn parse_time(minute_str: &str, seconds_str: &str) -> Option<i32> {
    if let Ok(minites) = i32::from_str(minute_str) {
        if let Ok(seconds) = i32::from_str(seconds_str) {
            return Some(minites * 60 + seconds);
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use crate::util::parse_rows;
    #[test]
    fn it_works() {
        let lyric_str = "
        [00:00.000] 作词 : 黄伟文\n[00:01.000] 作曲 : 雷颂德/Ted Lo\n[00:08.41]\n[00:11.70]就算只谈一场感情\n[00:15.70]除外都是一时虚荣\n[00:19.54]不等于在蜜月套房游玩过\n[00:23.77]就可自入自出仙境\n[00:27.75]情愿获得你的尊敬\n[00:31.63]承受太高傲的罪名\n[00:35.53]挤得进你臂弯 如情怀渐冷\n[00:39.59]未算孤苦也伶仃\n[00:42.76]明知爱\n[00:44.27]这种男孩子\n[00:45.98]也许只能如此\n[00:48.15]但我会成为你\n[00:49.27]最牵挂的一个女子\n[00:52.14]朝朝暮暮让你\n[00:53.71]猜想如何驯服我\n[00:55.88]若果亲手抱住\n[00:57.93]或者不必如此\n[01:00.21]许多旁人说我\n[01:01.74]不太明了男孩子\n[01:04.22]不受命令就是\n[01:05.52]一种最坏名字\n[01:08.24]笑我这个毫无办法\n[01:10.25]管束的野孩子\n[01:13.67]连没有幸福都不介意\n[01:27.76]若我依然坚持忠诚\n[01:31.58]难道你又适合安定\n[01:35.74]真可惜\n[01:36.64]说要吻我的还未吻\n[01:39.84]自己就自梦中苏醒\n[01:43.77]离场是否有点失敬\n[01:47.81]还是更轰烈的剧情\n[01:51.55]必需有这结果\n[01:53.76]才能怀念我\n[01:55.41]让我于荒野驰骋\n[01:58.74]明知爱\n[01:59.68]这种男孩子\n[02:01.93]也许只能如此\n[02:04.07]但我会成为\n[02:05.16]你最牵挂的一个女子\n[02:08.17]朝朝暮暮\n[02:09.22]让你猜想如何驯服我\n[02:11.99]若果亲手抱住\n[02:13.88]或者不必如此\n[02:16.21]许多旁人\n[02:17.25]说我不太明了男孩子\n[02:20.15]不受命令\n[02:21.29]就是一种最坏名字\n[02:24.16]笑我这个毫无办法\n[02:26.25]管束的野孩子\n[02:29.66]连没有幸福都不介意\n[02:46.75]明知爱\n[02:47.88]这种男孩子\n[02:49.92]也许只能如此\n[02:52.00]但我会成为\n[02:53.27]你最牵挂的一个女子\n[02:56.16]朝朝暮暮\n[02:57.24]让你猜想如何驯服我\n[03:00.00]若果亲手抱住\n[03:01.95]或者不必如此\n[03:04.16]许多旁人\n[03:05.12]说我不太明了男孩子\n[03:08.13]不受命令\n[03:09.10]就是一种最坏名字\n[03:12.14]我也笑我原来\n[03:13.62]是个天生的野孩子\n[03:17.81]连没有幸福都不介意\n
        ";

        let rows = parse_rows(lyric_str);
        assert_eq!(rows.len(), 69);
    }
}
