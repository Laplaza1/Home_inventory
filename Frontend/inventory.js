function isObject(value){
    console.log(value+" :"+ `${typeof value}`)
    if (typeof value === 'object'){
        Object.keys(value).forEach(item =>{ if (typeof item == 'object'){Object.keys(item).forEach(objectInobject => {
           console.log(`This is orbjec in object${value[item]}:${objectInobject}`) 
        });} else {console.log("item "+ item); console.log(`${item} item ${value[item]}`)}})
            
        
    }
    else return value

}


document.addEventListener("DOMContentLoaded",()=>
    {
        fetch("https://home-inventory-bml1.onrender.com/item")
        .then(Response =>Response.json())
        .then(data => 
            {
                data.forEach(element => 
                    {

                        console.log(element)
                        
                       Object.keys(element).forEach(key => {
                        console.log(`${key}: ${isObject(element[key])}`);
                    })
                    }
                );
            }
        )
    }) 




