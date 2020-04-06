
# Known Issues

1) Runner/Base Value Data
    - When there are multiple records for one one batter in a single event/action, it's difficult to assess which one is the most accurate.
    - Ideally, we should be looking for the farthest base that is recorded for that runner, but that is difficult. For now, it takes the last
      record, but this should be fixed. I *think* taking the record with the highest end_base_value will likely be more accurate.
    - For now, we're going to just hard code in a "fix" (read: hack) that changes any base value of 8 to 7.

## Issues that are fixed for the most part

1) Fix the games with broken metadata - DONE
2) Fix the games with missing defense. DONE
3) Sometimes the Runner Vec has more than 4 entries, causing crazy "base value start" values and base" value end
        a) *** BASE VALUE START looks like it's totally wrong, need to investigate. DONE
