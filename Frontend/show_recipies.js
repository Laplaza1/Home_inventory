


class data_store {
    constructor(type,data) {
        
        this.type = type
        this.data = data
    }
    async pull_data(url,map=false) 
        {

            try 
                {
                    let response = await fetch(url)     
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





document.addEventListener("DOMContentLoaded",async (event)=>{
    console.log("recipe test")
    await recipes.pull_data("https://home-inventory-bml1.onrender.com/recipe")
    console.log(recipes.data)

    console.log("Items test")
    await items.pull_data("https://home-inventory-bml1.onrender.com/item",map=true)
    
    await check_availability(recipes.data[0].ingredients[0].item_name,10)

    recipes.data.forEach(async (each)=>
        {
            each.ingredients.forEach(async (each_ingredient) => 
                {
                    console.log(each_ingredient.item_name)
                    console.log(await check_availability(each_ingredient.item_name,each_ingredient.quantity))

                });


        })

    

})