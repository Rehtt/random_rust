const SFZ_CHECK_CODE: &[u8] = b"10X98765432";
const SFZ_FACTOR: &[i32] = &[7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
const SFZ_A: &[&str] = &["11", "12", "13", "14", "15", "21", "22", "23", "31", "32", "33", "34", "35", "36", "37", "41", "42", "43", "44", "45", "46", "50", "51", "52", "53", "54", "61", "62", "63", "64", "65", "71", "81", "82"];
const PHONE_HEAD: &[&str] = &["139", "138", "137", "136", "135", "134", "159", "158", "157", "150", "151", "152", "188", "187", "182", "183", "184", "178", "130", "131", "132", "156", "155", "186", "185", "176", "133", "153", "189", "180", "181", "177"];
const XING: &[char] = &['李', '王', '张', '刘', '陈', '杨', '黄', '赵', '周', '吴', '徐', '孙', '朱', '马', '胡', '郭', '林', '何', '高', '梁', '郑', '罗', '宋', '谢', '唐', '韩', '曹', '许', '邓', '萧', '冯', '曾', '程', '蔡', '彭', '潘', '袁', '于', '董', '余', '苏', '叶', '吕', '魏', '蒋', '田', '杜', '丁', '沈', '姜', '范', '江', '傅', '钟', '卢', '汪', '戴', '崔', '任', '陆', '廖', '姚', '方', '金', '邱', '夏', '谭', '韦', '贾', '邹', '石', '熊', '孟', '秦', '阎', '薛', '侯', '雷', '白', '龙', '段', '郝', '孔', '邵', '史', '毛', '常', '万', '顾', '赖', '武', '康', '贺', '严', '尹', '钱', '施', '牛', '洪', '龚', '汤', '陶', '黎', '温', '莫', '易', '樊', '乔', '文', '安', '殷', '颜', '庄', '章', '鲁', '倪', '庞', '邢', '俞', '翟', '蓝', '聂', '齐', '向', '申', '葛', '岳'];
const MING: &[char] = &['伟', '刚', '勇', '毅', '俊', '峰', '强', '军', '平', '保', '东', '文', '辉', '力', '明', '永', '健', '世', '广', '志', '义', '兴', '良', '海', '山', '仁', '波', '宁', '贵', '福', '生', '龙', '元', '全', '国', '胜', '学', '祥', '才', '发', '武', '新', '利', '清', '飞', '彬', '富', '顺', '信', '子', '杰', '涛', '昌', '成', '康', '星', '光', '天', '达', '安', '岩', '中', '茂', '进', '林', '有', '坚', '和', '彪', '博', '诚', '先', '敬', '震', '振', '壮', '会', '思', '群', '豪', '心', '邦', '承', '乐', '绍', '功', '松', '善', '厚', '庆', '磊', '民', '友', '裕', '河', '哲', '江', '超', '浩', '亮', '政', '谦', '亨', '奇', '固', '之', '轮', '翰', '朗', '伯', '宏', '言', '若', '鸣', '朋', '斌', '梁', '栋', '维', '启', '克', '伦', '翔', '旭', '鹏', '泽', '晨', '辰', '士', '以', '建', '家', '致', '树', '炎', '德', '行', '时', '泰', '盛', '雄', '琛', '钧', '冠', '策', '腾', '楠', '榕', '风', '航', '弘', '秀', '娟', '英', '华', '慧', '巧', '美', '娜', '静', '淑', '惠', '珠', '翠', '雅', '芝', '玉', '萍', '红', '娥', '玲', '芬', '芳', '燕', '彩', '春', '菊', '兰', '凤', '洁', '梅', '琳', '素', '云', '莲', '真', '环', '雪', '荣', '爱', '妹', '霞', '香', '月', '莺', '媛', '艳', '瑞', '凡', '佳', '嘉', '琼', '勤', '珍', '贞', '莉', '桂', '娣', '叶', '璧', '璐', '娅', '琦', '晶', '妍', '茜', '秋', '珊', '莎', '锦', '黛', '青', '倩', '婷', '姣', '婉', '娴', '瑾', '颖', '露', '瑶', '怡', '婵', '雁', '蓓', '纨', '仪', '荷', '丹', '蓉', '眉', '君', '琴', '蕊', '薇', '菁', '梦', '岚', '苑', '婕', '馨', '瑗', '琰', '韵', '融', '园', '艺', '咏', '卿', '聪', '澜', '纯', '毓', '悦', '昭', '冰', '爽', '琬', '茗', '羽', '希', '欣', '飘', '育', '滢', '馥', '筠', '柔', '竹', '霭', '凝', '晓', '欢', '霄', '枫', '芸', '菲', '寒', '伊', '亚', '宜', '可', '姬', '舒', '影', '荔', '枝', '丽', '阳', '妮', '宝', '贝', '初', '程', '梵', '罡', '恒', '鸿', '桦', '骅', '剑', '娇', '纪', '宽', '苛', '灵', '玛', '媚', '琪', '晴', '容', '睿', '烁', '堂', '唯', '威', '韦', '雯', '苇', '萱', '阅', '彦', '宇', '雨', '洋', '忠', '宗', '曼', '紫', '逸', '贤', '蝶', '菡', '绿', '蓝', '儿', '翠', '烟', '小', '轩'];

use std::ops::{Add, Sub};
use chrono::{DateTime, Duration, Local};
use rand::Rng;

#[macro_export]
macro_rules! rand_array {
    ($array:expr) => {
        {
            $array[rand::thread_rng().gen_range(0..$array.len())]
        }
    };
}

pub fn rand_name() -> String {
    (0..rand::thread_rng().gen_range(2..=3))
        .map(|i| {
            if i == 0 {
                return rand_array!(XING);
            }
            rand_array!(MING)
        })
        .collect::<String>()
}

pub fn rand_phone() -> String {
    String::from(rand_array!(PHONE_HEAD))
        .add(format!("{:08}", rand::thread_rng().gen_range(0..99999999)).as_str())
}

pub fn rand_id_no(birthday: DateTime<Local>) -> String {
    let out = String::from(rand_array!(SFZ_A))
        .add(format!("{:02}", rand::thread_rng().gen_range(0..70)).as_str())
        .add(format!("{:02}", rand::thread_rng().gen_range(0..99)).as_str())
        .add(birthday.format("%Y%m%d").to_string().as_str())    // 生日码
        .add(format!("{:03}", rand::thread_rng().gen_range(0..999)).as_str()); // 顺序码

    let mut weighted = 0i32;
    for (i, &x) in out.as_bytes().iter().enumerate() {
        weighted += (x.to_string().parse::<i32>().unwrap() - 48) * SFZ_FACTOR[i]
    }
    out.add((SFZ_CHECK_CODE[(weighted as usize) % 11] as char).to_string().as_str())
}

pub fn rand_time(from: DateTime<Local>, to: DateTime<Local>) -> DateTime<Local> {
    from.add(Duration::seconds(rand::thread_rng().gen_range(1..to.sub(from).num_seconds())))
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Local};
    use super::*;

    #[test]
    fn it_works() {
        let a: DateTime<Local> = Local::now();
        println!("{}", rand_time(a.sub(Duration::days(180)),a));
        println!("{}", rand_id_no(a));
        println!("{}", rand_phone());
        println!("{}",rand_name());
    }
}
