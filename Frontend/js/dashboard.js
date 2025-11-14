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
                            let pendapp = document.getElementById("pendApper").cloneNode(true)
                            pendapp.id = "pendApp"
                            pendapp.className = "container"
                            pendapp.querySelector("#userName").textContent = atomo.username
                            pendapp.querySelector("#email").textContent = atomo.email
                            pendapp.querySelector("#home").textContent = atomo.home
                            pendapp.querySelector("#passWord").textContent = atomo.password
                            pendapp.querySelector("#phoneNumber").textContent = atomo.phone_number
                            pendapp.querySelector("#Reason").textContent = atomo.reason
                            document.getElementById("pendApps").appendChild(pendapp)
                        })


                    console.log(data)
                    console.log(data.Item_count)
                    Object.entries(data.Item_count).forEach(item=>{console.log(item);console.log(yValues[xValues.indexOf(item[0])]);yValues[xValues.indexOf(item[0])]=item[1];console.log(yValues[xValues.indexOf(item[0])]);})
                    document.querySelectorAll("#approve").forEach(async (a) => 
                        {
                            a.addEventListener("click",async (event) => 
                                {
                                    let body = {}
                                console.log(Array.from(event.target.parentElement.children).forEach(child=>
                                    {
                                        if (child.textContent)
                                            {
                                                console.log(String(child.textContent).trim())
                                                body[(String(child.id).toLowerCase())] = String(child.textContent)
                                                
                                            }
                                        
                                        else
                                            console.log(" is not real")
                                    }))
                                
                                console.log(body,document.cook)
                                //let  response = fetch(`https://home-inventory-bml1.onrender.com/user`,
                                let  response = fetch(`http://localhost:3000/user`, 
                                    {
                                        method:"POST",
                                        credentials:"include",
                                        headers: 
                                            {
                                                "Cookie":document.cookie,
                                                "Content-Type": "application/json"
                                            },
                                        body: JSON.stringify(body)
                                    })
                                    console.log(response)
                                            })
                        })
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