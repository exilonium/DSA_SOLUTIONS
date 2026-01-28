impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stk: Vec<i32> = vec![];

        for stone in asteroids {
            let mut alive = true;
            while let Some(&top) = stk.last() {
                if top > 0 && stone < 0 {
                    if top < -stone {
                        stk.pop();
                        continue;
                    } else if top == -stone {
                        stk.pop();
                        alive = false;
                        break;
                    } else {
                        alive = false;
                        break;
                    }
                } else {
                    break;
                }
            }

            if alive {
                stk.push(stone);
            }
        }

        stk
    }
}
