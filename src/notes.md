Putting some random notes in here to not clutter up other parts.

Ideally, we'd like to download all the player/coach/umpire bio info in one go. We can do this for players by doing a query
that looks like this: http://statsapi.mlb.com/api/v1/sports/{sport_id}/players/?season={season}, however if we want to "hydrate" the fields at all, we'd can't do it all in one go

http://statsapi.mlb.com/api/v1/people/545361?hydrate=xrefId will also give info on IDs for other platforms

Other Hyrdrate fields: transactions - this will give us detailed data on injuries, trades, options, etc. Will likely require a lot of text-analysis to get useful data out of it.

To get player changes - we can get that from: http://lookup-service-prod.mlb.com/json/named.ops_player_changes.bam?change_date=20191205 might be able to use these for historical player weights, or might be easier to go to
the old gameday XML files for this. **UPDATE - This is totally useless

Unfortunately, we can't do this for coaches and umpires, but those should be a relatively smaller proportion of the pull.

For the umpires, we'll want to track the HP Umpire only (for now) and keep bio info on their height and dob, as I'm curious if either of those have any impact on strike calling.
For coaches, we'll keep track of the Manager, Hitting Coach and Batting Coach, as well as bio on their dob and whether they were drafted, or played in the MLB.

For Venues, we can get detailed info such as capacity and dimensions from: https://statsapi.mlb.com/api/v1/venues/2681?hydrate=location,fieldInfo&season={season_ID}

!!! Need to find all the Hydrate fields, might be some buried info there.

To find all the hydrate fields use ?hydrate=hydrations

Awards can be found at: http://statsapi.mlb.com/api/v1/awards ()


Japanese data: http://npb.jp/bis/eng/players/

