document.addEventListener("DOMContentLoaded",(()=>
    {
        document.getElementById("signupForm").addEventListener("submit",((SubmitEvent)=>
            {
                console.log("Submitted",SubmitEvent)
                const form = document.getElementsByTagName("form")[0];
                const form_Data = new FormData(form);
                //let form_Data= new FormData(Object(SubmitEvent))
                console.log("Form data: ",JSON.stringify(form_Data))
                console.log(SubmitEvent)
                form_Data.forEach((value, key) => 
                    {
                        console.log(key,value)
                    })
                let  response = fetch(`https://home-inventory-bml1.onrender.com/pending`, 
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


            }))






    }))