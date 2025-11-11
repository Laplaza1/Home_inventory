document.addEventListener("DOMContentLoaded",(()=>
    {
        document.getElementById("signupForm").addEventListener("submit",((SubmitEvent)=>
            {
                let ls = document.getElementById("loading-screen")
                console.log("Submitted",SubmitEvent)
                const form = document.getElementsByTagName("form")[0];
                const form_Data = new FormData(form);
                //let form_Data= new FormData(Object(SubmitEvent))
                let y = {}
                ls.style.display = 'Flex'
                form_Data.forEach((value, key) => 
                    {
                        y[key] =value
                    })
                
                //let  response = fetch(`https://home-inventory-bml1.onrender.com/pending`,
                let  response = fetch(`http://localhost:3000/pending`, 
                    {
                        method:"POST",
                        headers: 
                            {
                                "Content-Type": "application/json"
                            },
                        body: JSON.stringify(y)
                    }).then((req,res)=>
                        {
                            if (req.status!=200)
                                {
                                    console.log(req.status)
                                }
                        
                            else
                                {
                                    
                                    location.reload()
                                    ls.style.display = "None"
                                }
                        })


            }))






    }))