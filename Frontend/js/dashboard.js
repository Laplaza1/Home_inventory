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
                    data.Pending_users.forEach(atomo =>
                        {
                            console.log(atomo)
                            let pendapp = document.getElementById("pendApp").cloneNode(true)
                            pendapp.className = "container"
                            pendapp.querySelector("#userName").textContent = atomo.username
                            pendapp.querySelector("#email").textContent = atomo.email
                            pendapp.querySelector("#passWord").textContent = atomo.password
                            pendapp.querySelector("#phoneNumber").textContent = atomo.phone_number
                            pendapp.querySelector("#Reason").textContent = atomo.reason
                            document.getElementById("pendApps").appendChild(pendapp)
                        })


                    console.log(data)
                    console.log(data.Item_count)
                    Object.entries(data.Item_count).forEach(item=>{console.log(item);console.log(yValues[xValues.indexOf(item[0])]);yValues[xValues.indexOf(item[0])]=item[1];console.log(yValues[xValues.indexOf(item[0])]);})
                } catch (error) {
                    console.log(error)
                }
                myChart.update()
            })
         document.querySelectorAll("#collapse_button").forEach(async (e)=>
                {
                    console.log(e)
                    e.addEventListener("click",async (event)=>
                        {
                            console.log("clicked")   
                            let x = event.target.parentElement.querySelector("#pendApps").style.display
                            
                            if(x=="none")
                                {
                                    event.target.parentElement.querySelector("#pendApps").style.display="block"

                                }
                            else
                                {
                                    event.target.parentElement.querySelector("#pendApps").style.display = "none"
                                }
                        })
                })



    }))