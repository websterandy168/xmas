fn main() {

    
   let  refrane = ["a partridge in a pair tree.", "two turtle-doves", "three French Hens", "four calling birds", "FIVE GOLDEN RINGS", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"
   ];
   for day in 0..12{
        println!("On the {}th Day of Christmas my true love gave to me", day+1);
        for i in (0..day+1).rev(){
            if (i == 0) & (day > 0){
                println!("and a")
            }
            println!("{}", refrane[i])

        }
        
   }

}
