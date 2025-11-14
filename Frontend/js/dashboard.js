document.addEventListener("DOMContentLoaded",(()=>
    {
        let x = fetch("https://home-inventory-bml1.onrender.com/admin_data"
        //let  x = fetch(`http://localhost:3000/admin_data`
            ,{method:"Get",credentials:"include",headers:{"Cookie":document.cookie}})
            .then((Response)=>Response.json())
            .then(data => 
            {
                try {
                    let nm = [0,1,2,3,4,56,6]
                    document.getElementById("itemNums").textContent = data.Number_of_homes
                    console.log(data)
                    console.log(data.Item_count)
                    Object.entries(data.Item_count).forEach(item=>{console.log(item);console.log(yValues[xValues.indexOf(item[0])]);yValues[xValues.indexOf(item[0])]=item[1];console.log(yValues[xValues.indexOf(item[0])]);})
                } catch (error) {
                    console.log(error)
                }
                myChart.update()
            })
        
        




    }))