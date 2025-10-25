fetch("https://home-inventory-bml1.onrender.com/recipe",(async (e)=>{
    console.log(e)
    await e
    .then((Response) => {
        Response.json()
        
    }).catch((err) => {
        console.log(err);
        alert(err)
    });

}))


fetch("https://home-inventory-bml1.onrender.com/item",(async (e)=>{
    console.log(e)
    await e
    .then((Response) => {
        Response.json()
        
    }).catch((err) => {
        console.log(err);
        alert(err)
    });

}))