Generate nodejs with playwright for the task below to get the data from the target link.



Target link : https://projecteuler.net/archives



Task :

In a target link, there is many tr elements. 

Each tr element has 3 td elements. 

The first td element is the number of the problem. 

The second td element is the title of the problem. 

The third td element is the number of people who solved the problem.



For each tr element, we need to get the first td element and the second td element. 

Scrap element from first td element and second td element. 



Then convert the scraped element to human readable. 



Rewrite the code above so it will iterate to scrap from the link strating from page 1 until page 17.


https://projecteuler.net/archives;page=1

https://projecteuler.net/archives;page=2

https://projecteuler.net/archives;page=3

until

https://projecteuler.net/archives;page=17