use std::str::Chars;

use log::debug;

use crate::lib::lib::char_to_num;



pub fn scan_datastream(stream: &str) -> usize
{
    let mut dupes = [0u8; 26];
    let mut char_buf = stream.chars();
    let mut has_duplicate = false;

    let mut set_indices = prefill_buf(14, &mut dupes, &mut char_buf);
    let mut end_of_window = 14;
    let mut dupe_count: usize = 0;

    for i in 0..26
    {
        if dupes[i] > 1
        {
            has_duplicate = true;
            dupe_count += dupes[i] as usize - 1;
        }
    }

    debug!("Start condition");
    debug!("---------------");
    debug!("List of indices set during initialization: {:?}", set_indices);
    debug!("current end of window: {}", end_of_window);
    debug!("Count of duplicates: {}", dupe_count);

    while has_duplicate && !dupes.is_empty()
    {
        let first_in = set_indices.remove(0);
        debug!("Dequeue'd first in index: {}", first_in);
        dupes[first_in] -= 1;
        if dupes[first_in] > 0
        {
            dupe_count -= 1;
        }
        if let Some(next_char) = char_buf.next()
        {
            debug!("Testing character <{}>", next_char);
            let char_num = char_to_num(&next_char);
            end_of_window += 1;
            debug!("End of window: {}", end_of_window);
            dupes[char_num] += 1;
            if dupes[char_num] > 1
            {
                dupe_count += 1;
            }
            set_indices.push(char_num);
        }

        debug!("Dupe count: {}", dupe_count);

        if dupe_count == 0 { has_duplicate = false;}
    }

    return end_of_window;
}

fn prefill_buf(fill_count: usize, dupe_buf: &mut [u8], char_buf: &mut Chars) -> Vec<usize>
{
    let mut set_indices = Vec::<usize>::new();
    for _i in 0..fill_count
    {
        if let Some(next_char) = char_buf.next()
        {
            debug!("Testing character <{}>", next_char);
            let char_num = char_to_num(&next_char);
            set_indices.push(char_num);
            dupe_buf[char_num] += 1;
        }
        else
        {
            return set_indices;
        }
    }

    return set_indices;
}

#[cfg(test)]

#[test]
pub fn mjqjpqmgbljsphdztnvjfqwrcgsmlb()
{
    let test_line = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let end_of_start_of_packget = scan_datastream(test_line);

    assert_eq!(end_of_start_of_packget, 19);
}

#[test]
pub fn bvwbjplbgvbhsrlpgdmjqwftvncz()
{
    let test_line = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(scan_datastream(test_line), 23);
}

#[test]
pub fn nppdvjthqldpwncqszvftbrmjlhg()
{
    let test_line = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(scan_datastream(test_line), 23);
}

#[test]
pub fn nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg()
{
    let test_line = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(scan_datastream(test_line), 29);
}

#[test]
pub fn zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw()
{
    let test_line = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(scan_datastream(test_line), 26);
}