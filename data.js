
let xvalue = [];

let yvalue = [];




function daysInMonth(month, year) {
    switch (month) {
        case 1: // January
        case 3: // March
        case 5: // May
        case 7: // July
        case 8: // August
        case 10: // October
        case 12: // December
            return 31;
        case 4: // April
        case 6: // June
        case 9: // September
        case 11: // November
            return 30;
        case 2: // February
            return (year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0)) ? 29 : 28;
        default:
            return -1; // Invalid month
    }
}







let array

document.addEventListener("DOMContentLoaded",()=>
    {
        fetch("https://home-inventory-bml1.onrender.com/data").then((Response)=>Response.json()).then(data => 
            {
                console.log(data

                )
                data.forEach(element => 
                    {
                        if (element.item == "68a868023e401df96157c980")
                            {
                                xvalue.push(element.change);
                                let z = new Date(Number(element.date.$date.$numberLong))
                                array = Array.from({ length: daysInMonth(z.getMonth()) }, (_, i) => i + 1);
                                console.log(array)
                                yvalue.push(z); 
                            }
                        })
                    console.log("XValue:"+xvalue + "\n")
                    console.log("YValue:"+yvalue + "\n")}).then(()=>
                        {
                            console.log("Global: "+ red)
                            //red.pop()
                            red = xvalue
                            console.log("Global: "+ red)
                            myChart.data.datasets[0].data = red 
                            console.log("myChart "+myChart.data.datasets[0].data)
                            myChart.data.labels = array
                            console.log(myChart.data.labels)
                            myChart.update()
                        })
                    
       
        document.getElementById('updateButton').addEventListener('click', function() {
                // Example: Multiply the current red data by 1.5
                console.log(myChart)
                if (myChart.data.datasets.length > 1) {  // Optional: Prevent removing all datasets
                    myChart.data.datasets.splice(2, 1);  // Remove dataset at index 2 (Blue)
                    myChart.update();
        };})


    });
