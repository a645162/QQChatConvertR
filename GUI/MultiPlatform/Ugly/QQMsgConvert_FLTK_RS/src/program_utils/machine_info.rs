#![allow(dead_code)]
extern crate num_cpus;

// https://zhuanlan.zhihu.com/p/466389032
trait F32Utils {
    fn round_fixed(self, n: u32) -> f32;
}

impl F32Utils for f32 {
    fn round_fixed(self, n: u32) -> f32 {
        if n <= 0 {
            return self.round();
        }
        let i = 10_usize.pow(n) as f32;
        let x = self * i;

        if self > 0_f32 {
            // 正数情况 1.15_f32.round()  为 1.2
            let m = x.round() as u32;
            m as f32 / i
        } else {
            // -11.3_f32.floor() =-11 ;-11.6_f32.floor()=-11 ;-11.3_f32.ceil()=-12 ;-11.5_f32.round() = -12
            // 默认的负数 round 四舍五入取整  (a) -1.15_f32.round() 为 -1.2 (b)  ;  违背我们理解的向上取整b > a的普遍原意;
            // 比如js 中 四舍五入 Math.round(11.5) as a 的返回值是12, Math.round(-11.5) as b 的返回值是 -11 ;符合大众对取整的理解;
            let mr = x.trunc(); //整数部分
            let mf = x.fract(); //小数部分

            if mf.abs() >= 0.5 {
                // -3.14159 四舍五入保留3位 则-3141.59 /1000 = -3.141 59(逢五进一) 变为-3.140
                return (mr + 1_f32) / i;
            }
            // 小数部分  < 0.5 直接舍弃小数部分 ; 小数点直接使用整数部分向前移动n位
            mr / i
        }
    }
}


pub fn get_cpu_core_nums() -> usize {
    return num_cpus::get();
}

pub fn get_thread_nums() -> usize {
    (
        (
            get_cpu_core_nums() as f32 * (
                5.0 / 7.0
            )
        ).round_fixed(0)
    ) as usize
}