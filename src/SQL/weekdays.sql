create  or rePLACE function weekdays(p_date_begin date, p_date_end date)
    returns integer
    language sql
as
$$
select
    COUNT(*) FILTER(WHERE EXTRACT(dow FROM days) BETWEEN 1 AND 5)::INT AS weekdays
from generate_series( least( p_date_begin  , p_date_end ) , greatest(p_date_begin , p_date_end)  , interval '1 day') days

    $$;


/* https://www.codewars.com/kata/58241d05e7a162c5b100010f/train/sql
You need to create a function that calculates the number of weekdays (Monday through Friday) between two dates inclusively.

The function should be named weekdays accept two arguments of type DATE and return an INTEGER value.

weekdays(DATE, DATE) INTEGER
The order of arguments shouldn't matter. To illustrate both of the following queries

SELECT weekdays('2016-01-01', '2016-01-10');
SELECT weekdays('2016-01-10', '2016-01-01');
should produce the same result

 weekdays
----------
        6
(1 row)

*/