from sanic import Sanic, response

import os
import sys
from functools import partial
import json
from os.path import getctime
import datetime
from sanic.worker.loader import AppLoader

#app = Sanic("MyHelloWorldApp")

def attach_endpoints(app: Sanic):
    @app.get("/")
    async def hello_world(request):
        return response.text("Hello, world.")


    @app.get("/auth")
    async def auth(request):
        if (request.token):
            with open('logins.txt') as f:
                lines = f.readlines()
            for line in lines:
                print(line, request.token)
                if line == request.token:
                    print("ok")
                    return response.text("", status=200)
        
        return response.text("Error.\n", status=500)

    @app.get("/getdata")
    async def get_data(request):
        date = request.args.get("date") + ".json"
        directory = "/home/user/workspace/api/jsons/"
        with os.scandir(directory) as files:
            for file in files:
                rec = directory + file.name
                #rec_date = datetime.datetime.fromtimestamp(getctime(rec)).strftime('%Y-%m-%d')            
                if date == file.name:
                    with open(rec) as f:
                        data = json.load(f)
                    return response.json(data, status=200)
                
        return response.text("Error.\n", status=500)



    @app.route('/date')
    async def get_date(request):
        date = request.args.get('date')
        return response.text(date)
   
    
def create_app(app_name: str) -> Sanic:
    app = Sanic(app_name)
    attach_endpoints(app)
    return app


if __name__ == "__main__":
    loader = AppLoader(factory=partial(create_app, "test"))
    app = loader.load()
    app.prepare(host = "0.0.0.0", port=9999, dev=True)
    Sanic.serve(primary=app, app_loader=loader)    