from flask import Flask, request

app = Flask(__name__)

@app.route('/log', methods=['POST'])
def log():
    data = request.json
    print("GELEN LOG:", data)
    return {'status': 'ok'}, 200

if __name__ == '__main__':
    app.run(port=8000)
