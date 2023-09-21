import network
import usocket as socket
import ujson as json
from time import sleep
from picozero import Button, pico_temp_sensor, pico_led
import machine

#globals
ssid = 'SSID_WIFI'
password = 'PASSWD_WIFI'
led = machine.Pin(3, machine.Pin.OUT)

def connect():
    wlan = network.WLAN(network.STA_IF)
    wlan.active(True)
    wlan.connect(ssid, password)
    while wlan.isconnected() == False:
        sleep(0.1)
    ip = wlan.ifconfig()[0]
    return ip

def press_pc_switch(time):
    led.value(1)
    sleep(time)
    led.value(0)

    response = json.dumps([{'status': 'button pressed' }])
    conn.send('HTTP/1.1 200 OK\n')
    conn.send('Content-Type: application/json\n')
    conn.send('Content-Length: {}\n\n'.format(len(response)))
    conn.send(response)

def parse_request(request):
    method, path, version = request.split('\r\n')[0].split(' ')
    return method, path

def handle_request(conn, request):
    method, path = parse_request(request)
    if method == 'GET' and path == '/api/pc_on':
        press_pc_switch(0.1)
    if method == 'GET' and path == '/api/pc_off':
        press_pc_switch(1)
    else:
        conn.send('HTTP/1.1 404 Not Found\n')

try:
    ip = connect()
    print(ip)
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.bind(('0.0.0.0', 80))
    s.listen(5)
    while True:
        conn, addr = s.accept()
        request = conn.recv(1024).decode('utf-8')
        handle_request(conn, request)
        conn.close()
except KeyboardInterrupt:
    machine.reset()
