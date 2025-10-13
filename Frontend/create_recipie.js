const items = new Map()



document.addEventListener("DOMContentLoaded",()=>
    {
        
        document.querySelectorAll("#add").forEach((e)=>
            {
                e.addEventListener("click",(event)=>
                    {
                        console.log("Button Colicked")
                        console.log(event.target.parentElement.querySelector("#item"))
                        event.target.parentElement.querySelector("#itemList").appendChild(event.target.parentElement.querySelector("#item").cloneNode(true))
                        console.log(event.target.parentElement)
                    });
                    
                
                    
                
            })
            items.forEach((value,key,map)=>{console.log("test")})
            
    })
fetch("https://home-inventory-bml1.onrender.com/item").then((response)=>response.json()).then((data)=>
    {
        data.forEach((e)=>
            {
                items.set(e.item_name,e)
            });
            console.log(items.keys())
    }).then(()=>
        {
            const itemList = document.querySelectorAll("#itemList")
            if (!itemList)
                {
                    console.log("Item list is null")
                    return
                }
            else
                {
                    items.forEach((value,key,map)=>
                        {
                            console.log(value.category)
                            console.log(value.category =="Meat")
                            switch(value.category[0])
                                {
                                    case "Meat": 
                                        {
                                            return console.log("We have the Meats")
                                        };
                                    case "Spice": 
                                        {
                                            console.log("What is this? ");
                                            return console.log("Spicy")
                                        };
                                    case "Other": 
                                        {
                                            return console.log("What is this?")
                                        };
                                    default: 
                                        {
                                            return console.log("nothing")
                                        };

                                }
                        
                        })
           
                    console.log(items.keys())
                }
        
        })