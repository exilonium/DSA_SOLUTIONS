impl Solution {
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        if items.is_empty() || budget <= 0 {
            return 0;
        }

        let n = items.len();
        let mut min_price = i32::MAX;
        let mut max_factor = 0usize;

        for item in &items {
            min_price = min_price.min(item[1]);
            max_factor = max_factor.max(item[0] as usize);
        }

        let mut freq = vec![0i32; max_factor + 1];

        for item in &items {
            freq[item[0] as usize] += 1;
        }

        // Count factors that are multiples of each factor.
        for factor in 1..=max_factor {
            let mut multiple = factor * 2;

            while multiple <= max_factor {
                freq[factor] += freq[multiple];
                multiple += factor;
            }
        }

        // Keep profitable purchase opportunities.
        let mut opportunities: Vec<(i32, i32)> = Vec::new();

        for item in &items {
            let factor = item[0] as usize;
            let price = item[1];
            let free_count = freq[factor] - 1;

            if free_count > 0 && price <= 2 * min_price {
                opportunities.push((price, free_count));
            }
        }

        opportunities.sort_unstable_by_key(|x| x.0);

        let mut remaining = budget;
        let mut answer = 0;

        for (price, count) in opportunities {
            let take = count.min(remaining / price);

            answer += 2 * take;
            remaining -= take * price;
        }

        answer + remaining / min_price
    }
}
