
document.addEventListener("DOMContentLoaded",()=>
    {
        document.getElementById("priceInput").addEventListener('input',(event)=>
            {
                console.log(event.target.value)
                if (event.target.value <0)
                    {
                        event.target.value = event.target.value * -1 ;
                    }

            })
        if (document.getElementById("selectInput").value == "Select"|document.getElementById("nameInput").value == ""|document.getElementById("amountInput").value== ""|document.getElementById("priceInput").value==""|getSelectedCategories()==false)
            {

                 document.getElementById("selectInput").parentElement.querySelector("#submitButton").disabled= true
            }

        document.getElementById("selectInput").addEventListener("change",(e)=>
            {
                
                if (e.target.value == "Select")
                    {
                        e.target.parentElement.querySelector("#submitButton").disabled= true
                        console.log("Select:",e.target.parentElement.querySelector("#submitButton").disabled)

                    }
                else
                    {
                        e.target.parentElement.querySelector("#submitButton").disabled = false
                        console.log("Select:",e.target.parentElement.querySelector("#submitButton").disabled)
                    
                    }
            })
        document.getElementById("submitButton").addEventListener("click",(event)=>
            {
            
                const form = document.getElementsByTagName("form")[0];
                const formData = new FormData(form);
                console.log("Categories func",getSelectedCategories())
                formData.append("categories",[getSelectedCategories()])
                const formObject = {};

                // Convert FormData to a plain object for logging
                formData.forEach((value, key) => 
                    {
                        console.log(key)
                    if (key == "price")
                        {
                            console.log("found key: ", parseFloat(value));
                            value = parseFloat(value);
                            formObject[key] = parseFloat(value);
                        }

                    else{
                        
                        if (isNaN(Number(value)))
                        {
                            formObject[key] =value;
                            if (key == "time")
                            {
                                let dato = new Date(value.toString())
                                console.log(dato.getTime())
                                formObject["time"] = dato.getTime()
                                formData.set("time",dato.getTime())
                            }
                        }
                    else{
                    formObject[key] = Number(value);
                    console.log(key," ",Number(value),)
                    if (key == "time")
                        {
                            let dato = new Date(value.toString())
                            console.log(dato.getTime())
                            formObject["time"] = dato.getTime()
                            formData.set("time",dato.getTime())
                    }}
                }
                    })
                formObject["time"] = Number(new Date())
                console.log(Number(new Date()))
                console.log(formObject)
                let  response = fetch(`https://home-inventory-bml1.onrender.com/item`, 
                    {
                        method:"POST",
                        headers: 
                            {
                                "Content-Type": "application/json"
                            },
                        body: JSON.stringify(formObject)
                    }).then((req,res)=>
                        {
                            if (req.status!=200)
                                {
                                    console.log(req.status)
                                }
                        
                            else
                                {
                                    location.reload()

                                }
                        })
            })
                
        
        let time= document.getElementById("timeInput")
        time.value = new Date().getTime()
        
        document.getElementById("amountInput").addEventListener('input',(event)=>
            {
                console.log(event.target.value)
                if (event.target.value <0)
                    {
                        event.target.value = event.target.value * -1 ;
                    }

            })

        //price Input
        document.getElementById("priceInput").addEventListener('input',(event)=>
            {
                console.log(event.target.value)
                if (event.target.value <0)
                    {
                        event.target.value = event.target.value * -1 ;
                    }

            })

        
    })

    // Format to two decimal places
    // function formatDecimal() {
    //     let value = parseFloat(input.value) || 0;
    //     input.value = value.toFixed(2);
        
    // }


   

function toggleSelection(element) 
        {
            element.classList.toggle('selected');
        }

function getSelectedCategories() 
    {
        const buttons = document.querySelectorAll('.toggle-button.selected');
        const selectedOptions = Array.from(buttons).map(button => button.getAttribute('data-value'));
        console.log(typeof selectedOptions)
        return selectedOptions
    }

