

fn main() {
    let nums = vec![3,2,4];
    let target = 6;
    println!("{:?}", two_sum(nums, target));
}
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for item in 0..nums.len() - 1{
        //println!("item = {}", item);
        for i in item + 1..nums.len(){
            //println!("i = {}", i);
            if nums[item] + nums[i] == target{
                let ans = vec![item as i32, i as i32]; 
                return ans
            }
            //println!("{} + {} != {}",nums[item], nums[i], target);
        }
    }
    return vec![0];
}