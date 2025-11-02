


class data_store {
    constructor(type,data) {
        
        this.type = type
        this.data = data
    }
    async pull_data(url,map=false) 
        {

            try 
                {
                    let response = await fetch(url,{method:"Get",credentials:"include",headers:{"Cookie":document.cookie}})     
                    let vara = response.json() 
                    if (map)
                        {
                            let mapped = new Map
                            let x = await vara
                            x.forEach(element => {
                                mapped.set(element._id.$oid,element)
                            });
                            this.data = mapped
                        }
                    else
                        {
                            this.data = await vara
                        }
                } 
            catch (error) 
                {
                    console.log(error)
                }
        }

}


async function check_availability(item_name,quantity) {
    let ab =await items.data.get(item_name)
    return [ab.quantity>=quantity,ab.quantity-quantity]
}



let recipes = new data_store("recipe")







let items = new data_store("items")





document.addEventListener("DOMContentLoaded",async (evento)=>{
    console.log("recipe test")
    await recipes.pull_data("https://home-inventory-bml1.onrender.com/recipe")
    console.log(recipes.data)

    console.log("Items test")
    await items.pull_data("https://home-inventory-bml1.onrender.com/item",map=true)
    
    const loadingScreen = document.getElementById("loading-screen");
    const mainContent = document.getElementById("main-content");
    // console.log("info about .data",(recipes.data== true|items.data== true ))
        if (items.data !=null & recipes.data !=null)
            {
                
                Promise.allSettled([Promise.resolve(items.data)]).then((results)=>
                {
                
                    if (results[0].status=="fulfilled")
                        {
                                
                                console.log("Items Loaded")
                                Promise.allSettled([Promise.resolve(recipes.data)]).then((results)=>
                                {
                                
                                    if (results[0].status=="fulfilled")
                                        {
                                                
                                                console.log("Recipe Loaded")
                                                loadingScreen.style.display = "none";
                                                mainContent.style.display = "block";
                                        }  
                                })
                        }  
                })
                
            }
    await recipes.data.forEach(async (recipe)=>
        {

            let new_recipe = document.getElementById("hidden").querySelector('#recipe').cloneNode(true);

            let ava = []
            for(const ingred of recipe.ingredients)
                {
                    console.log(ingred)
                    let xy = await check_availability(ingred.item_name,ingred.quantity)
                    console.log(xy)
                    ava.push(xy[0]!=false)  
                    
                }
            
            new_recipe.querySelector("#availablility").querySelector("p").textContent = (ava.find(value => value === false)==undefined)                
            new_recipe.querySelector("#recipe_name").querySelector("p").textContent = recipe.recipe_name
            new_recipe.querySelector("#collapseButton").addEventListener("click",async (event)=>
                {
                      
                            let x = event.target.parentElement.querySelector("#collapse").style.display
                            
                            if(x=="none")
                                {
                                    
                                    event.target.parentElement.querySelector("#collapse").style.display="grid"
                                    // event.target.parentElement.querySelector("#collapse").style.animation = "fadeIn 2s ease-in-out"

                                }
                            else
                                {
                                    event.target.parentElement.querySelector("#collapse").style.display = "none"
                                }

                })
            new_recipe.querySelector("#Description").querySelector("p").textContent = recipe.Description
            
            new_recipe.querySelector("#time_to_cook").querySelector("p").textContent = recipe.time_to_cook

            recipe.steps.forEach((step,index)=>
                {
                    console.log(step,index)
                    let step_elem = document.createElement("label")
                    step_elem.id= index+1
                    step_elem.textContent = "Step " + (Number(index)+1)+" : ";
                    let step_elem_p = document.createElement("p")
                    step_elem_p.textContent = step
                    step_elem.appendChild(step_elem_p)
                    new_recipe.querySelector("#steps").appendChild(step_elem)
                })

            document.getElementById("Recipes").appendChild(new_recipe);
        })
        console.log(document.querySelectorAll("#collapseButton"))

        document.getElementById("searchInput").addEventListener("input",(event)=>
            {
                
                let search = event.target.value
                console.log(search)
                document.querySelectorAll("#recipe_name").forEach((ne)=>
                    {
                        if (ne.textContent.toLowerCase().startsWith(search.toLowerCase()))
                            {
                                
                                ne.parentElement.style.display = "grid"
                            }
                        
                        

                        else
                            {
                                ne.parentElement.style.display = "none";
                            }
                    })




            })
        document.getElementById("tagInput").addEventListener("change",(e)=>
            {
                let filter =e.target.value;
                document.querySelectorAll("#availablility").forEach((er)=>
                    {
                        console.log(er.querySelector("p").textContent)
                        let availability = er.querySelector('p')
                        if (availability.textContent== filter|filter == "Select"|filter=="All")
                            {
                                
                                er.parentElement.style.display = "grid"
                            }
                        
                        

                        else
                            {
                                er.parentElement.style.display = "none";
                            }
                        

                    })
                })

        document.querySelectorAll("#collapseButton").forEach(async (e)=>
                {
                    console.log(e)
                    e.addEventListener("click",async (event)=>
                        {
                            console.log("clicked")   
                            let x = event.target.parentElement.querySelector("#collapse").style.display
                            
                            if(x=="none")
                                {
                                    event.target.parentElement.querySelector("#collapse").style.display="grid"

                                }
                            else
                                {
                                    event.target.parentElement.querySelector("#collapse").style.display = "none"
                                }
                        })
                })
            
    

})