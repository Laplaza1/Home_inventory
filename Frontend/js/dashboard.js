document.addEventListener("DOMContentLoaded",(()=>
    {
        let x = fetch("https://home-inventory-bml1.onrender.com/admin_data"
        //let  x = fetch(`http://localhost:3000/admin_data`
            ,{method:"Get",credentials:"include",headers:{"Cookie":document.cookie}})
            .then((Response)=>Response.json())
            .then(data => 
            {
                try {
                    let nm = 0
                    Object.values(data.Item_count).forEach(elem=>nm+=elem)
                    document.getElementById("itemNums").textContent = nm
                    document.getElementById("homeNums").textContent = data.Number_of_homes
                    document.getElementById("userNums").textContent = data.Number_of_users
                    console.log(data)
                    console.log(data.Item_count)
                    Object.entries(data.Item_count).forEach(item=>{console.log(item);console.log(yValues[xValues.indexOf(item[0])]);yValues[xValues.indexOf(item[0])]=item[1];console.log(yValues[xValues.indexOf(item[0])]);})
                } catch (error) {
                    console.log(error)
                }
                myChart.update()
            })
         let response = fetch("https://home-inventory-bml1.onrender.com/pending",
        //let  response = fetch(`http://localhost:3000/admin_data`,
            {method:"Get",credentials:"include",headers:{"Cookie":document.cookie}})
            .then((Response)=>Response.json())




    }))