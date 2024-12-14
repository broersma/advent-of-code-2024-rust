# Prompts

My prompts in relevant ChatGPT conversations, offered here for purposes of education, entertainment, and accountability:

## Day 1

> How do I get this to work?
>
>          .lines()
>           .map(|l| l
>                .split("   ")
>                .map(|d| d.parse::<u32>())
>               .collect::<Vec<u32>>())

---

>     fn count_num(list: &Vec<u32>, num: u32) -> u32 {
>         list.into_iter().filter(|&a| *a == num).count()
>     }

> Why into_iter or iter?

## Day 6

> How do I make a union of hashSet and a new element in Rust and then get a new hashset?

> How about the union function on hashset?