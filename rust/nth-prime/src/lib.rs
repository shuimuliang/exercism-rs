static mut pre_calced: bool = false; // 是否预先计算素数

static mut prime: [u32; 100] = [0; 100]; // 存储素数
static mut primecount: u32 = 0; // 素数个数

//TODO: 多线程问题

fn calc_prime_array() {
    unsafe {

        prime[0] = 2;
        primecount = 1;

        // n 个素数
        for i in 1..10 {
            // 从最近一个素数+1开始, 向后逐个递增，找未整除过的素数
            for j in prime[i-1]+1.. {
                // 扫描之前的素数，都不能整除，则标记
                let mut hit = false;
                for k in 0..primecount {
                    if j % prime[k as usize] == 0 {
                        hit = true;
                        break;
                    }
                }
                if hit == false {
                    prime[i]=j;
                    primecount += 1;
                    break;
                }
            }
        }
    }
}

pub fn nth(n: u32) -> u32 {
    unsafe {
        if pre_calced == false {
            calc_prime_array();
            pre_calced = true;
        }
        prime[n as usize]
    }
}
