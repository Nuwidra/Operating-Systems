# ======================================================================================
# A module used to execute new code and
# applications by creating new processes
# ======================================================================================

import subprocess

# ======================================================================================
# Provides several functions and variables that are used
# to manipulate different parts of the Python runtime environment.
# ======================================================================================
import sys
import threading

# ======================================================================================
# The filename is declared which will be position 0
# ======================================================================================

filename = sys.argv[0] # stress

# ======================================================================================
# The option will be -in which will be in position 1
# ======================================================================================
option = sys.argv[1]    # -n

# ======================================================================================
# The number of the threads will be in position 2
# ======================================================================================
number_threads = int(sys.argv[2]) # 1000

# ======================================================================================
# The httpclient will be position 3 of the arguments
# ======================================================================================
HTTPclient = sys.argv[3]    # hhtpclient -h 

# ======================================================================================
# The httpclient_opion will be -h in position 4 of the arguments
# ======================================================================================
HTTPclient_option = sys.argv[4]   # -h

# ======================================================================================
# The HTTPclient host will be the server port in position 5 of the arguments.
# ======================================================================================
HTTPclient_host = sys.argv[5] # 8080

# ======================================================================================
# The additional_option will be the option as is the get example
# ======================================================================================
additional_option = sys.argv[6] # get

# ======================================================================================
# The resource will be the file in html extension
# ======================================================================================
resource = sys.argv[7]  # index.hmtlHTTPclient_option

# ======================================================================================
# The client_parameters will be the set of HTTPclient, HTTPclient_option, HTTPclient_host, additional_option and resource
# ======================================================================================
client_parameters = [HTTPclient, HTTPclient_option, HTTPclient_host, additional_option, resource]

# ======================================================================================
# The attack function will consist of executing the thread and with run
# ======================================================================================
def attack(client_parameters):

    subprocess.run(client_parameters)


command_HTTP_client = [filename, " ", option, " ", number_threads, " ", HTTPclient, " ", client_parameters]

# ======================================================================================
# This loop will execute multiple threads
# ======================================================================================
for i in range(number_threads):
    thread = threading.Thread(target=attack, args=(client_parameters, ))
    thread.start()  