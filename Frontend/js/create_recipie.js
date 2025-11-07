const items = new Map()


class item {
    constructor(item_name,quantity,method_of_Measure) {
    this.item_name = item_name;
    this.quantity = quantity;
    this.method_of_Measure = method_of_Measure
        
    }


}


let xyz
try{
    xyz=fetch("https://home-inventory-bml1.onrender.com/item",{method:"Get",credentials:"include",headers:{"Cookie":document.cookie}}).then((response)=>response.json()).then((data)=>
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
                                            let x = document.createElement("option")
                                            x.text = key
                                            document.querySelector("#Meats").querySelector("#item").querySelector("#itemselectInput").appendChild(x)
                                            console.log(document.querySelector("#Meats").querySelector("#item"))
                                            return console.log("We have the Meats")
                                        };
                                    case "Spice": 
                                        {
                                            let x = document.createElement("option")
                                            x.text = key
                                            document.querySelector("#Spices").querySelector("#item").querySelector("#itemselectInput").appendChild(x)
                                            console.log("What is this? ");
                                            return console.log("Spicy")
                                        };
                                    case "Dairy":
                                        {
                                            let x = document.createElement("option")
                                            x.text = key
                                            document.querySelector("#Dairy").querySelector("#item").querySelector("#itemselectInput").appendChild(x)
                                            return console.log("This is Dairy")
                                        }
                                    case "Other": 
                                        {
                                            let x = document.createElement("option")
                                            x.text = key
                                            document.querySelector("#Other").querySelector("#item").querySelector("#itemselectInput").appendChild(x)
                                            return console.log("What is this?")
                                        };
                                    default: 
                                        {
                                            return console.log("nothing")
                                        };

                                }
                        
                        })
           
                    //console.log(items.keys())
                }
        
        })}catch{
            console.log("fetch failed")
        }

document.addEventListener("DOMContentLoaded",()=>
    {
        const loadingScreen = document.getElementById("loading-screen");
        const mainContent = document.getElementById("main-content");
        if (xyz)
            {
                Promise.allSettled([Promise.resolve(xyz)]).then((results)=>
                {
                
                    if (results[0].status=="fulfilled")
                        {
                                
                                console.log("Loaded")
                                loadingScreen.style.display = "none";
                                mainContent.style.display = "block";
                        }  
                })
                
            }
        
        document.querySelectorAll("#quantity").forEach((o)=>
            {
                o.addEventListener("input",(evento)=>
                    {
                        let v = evento.target.value
                        if (v<0)
                            {
                                evento.target.value = v*-1
                            }
                        
                    })
            })
        document.querySelectorAll("#addStep").forEach((e)=>
            {
                e.addEventListener("click",(event)=>
                    {
                        console.log("Button Colicked")
                        
                        let y = event.target.parentElement.querySelector("#steplist")
                        
                        let x = event.target.parentElement.querySelector("#step").cloneNode(true)
                        x.querySelector("#stepText").value = ""
                        count =document.querySelectorAll("#step").length
                        x.querySelector("#stepLabel").textContent = count+1
                        event.target.parentElement.querySelector("#steplist").appendChild(x)
                        
                    });
            })    
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
        document.querySelectorAll("#removeStep").forEach((e)=>
            {
                e.addEventListener("click",(event)=>
                    {

                        let last_elem = document.querySelectorAll("#step").length
                        
                        if (last_elem>1)
                            {
                                console.log("Remove step Button Colicked");
                                document.querySelectorAll("#step")[last_elem-1].remove();
                            }
                        else
                            {
                                console.log("Stop trying to remove the only remaining item field for this type")
                            }
                    });
                })
            //Reset button
            document.getElementById("reset").addEventListener("click",(event)=>
                {
                    let items = document.querySelectorAll("#item")
                    items.forEach((item)=>
                        {
                            item.childNodes.forEach((child)=>
                                
                                {
                                    console.log(child)
                                    //child.textContent=""
                                    child.value = "Select"
                                })


                        })


                })
            
                //Remove Buttons
            document.querySelectorAll("#Remove").forEach((e)=>
            {
                e.addEventListener("click",(event)=>
                    {

                        let last_elem = event.target.parentElement.querySelectorAll("#item").length
                        if (last_elem>1)
                            {
                                console.log("Remove Button Colicked");
                                event.target.parentElement.querySelectorAll("#item")[-1].remove(true);
                            }
                        else
                            {
                                console.log("Stop trying to remove the only remaining item field for this type")
                            }
                    });
                    
                
                    
                
            })
            document.getElementById("submit").addEventListener("click",(event)=>
                {

                    let recipe = 
                    {
                        recipe_name:event.target.parentElement.querySelector("#nameInput").value,
                        ingredients:[],
                        steps:[],
                        time_to_cook:document.getElementById("prepTime").value,
                        description: document.querySelector("#descriptionInput").value

                    }
                    document.querySelectorAll("#item").forEach((item)=>
                        {
                            //Uncomment this to log the type and each item
                            // console.log(item.parentElement.parentElement)
                            // console.log("This is the item",item)
                            let itemo = []
                            item.childNodes.forEach((itemChildren)=>
                                {
                                    if (!itemChildren.value | (itemChildren.value=="Select"))
                                        {    
                                            return
                                        }
                                    else
                                        {
                                            //Uncomment to see the childNode and the childNode value
                                            // console.log("This is passing Children of item",itemChildren)
                                            // console.log(itemChildren.value)

                                            itemo.push(itemChildren.value)
                                        }
                                })
                            if (!itemo | (itemo.length<3))
                                {
                                    return
                                }
                            else
                                {
                                    //Uncomment to Check the first value
                                    // console.log("Itemo",itemo)
                                    // console.log(itemo[0])
                                    itemo[0]= items.get(itemo[0])["_id"]["$oid"]
                                    recipe.ingredients.push(itemo)
                                }
                        })
                    if (recipe.ingredients.length<1){console.log("Theres no ingredients to this recipe");return}
                    document.querySelectorAll("#step").forEach((step)=>
                        {
                            //Uncomment to check steps and values
                            // console.log(step)
                            // console.log(step.querySelector("#stepText").value);
                            recipe.steps.push(step.querySelector("#stepText").value)
                        })
                    console.log(recipe)
                    fetch("http://localhost:3000/recipe", {
                        method:"POST",
                        headers: 
                            {
                                "Content-Type": "application/json"
                            },
                        body: JSON.stringify(recipe)
                    }).then((response)=>
                        {

                        // Uncomment to check response
                        // console.log(response.json())
                        // console.log(recipe)})
                            if (!response.ok)
                                {
                                    alert("Recipe Failed to be created! Check your values and try again.")

                                }
                        })
                    
                })
            
            
    })




  
  