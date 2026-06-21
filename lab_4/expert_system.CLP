(deffunction ask-question (?question $?allowed-values)
   (printout t ?question)
   (bind ?answer (read))
   (if (lexemep ?answer) 
       then (bind ?answer (lowcase ?answer)))
   (while (not (member ?answer ?allowed-values)) do
      (printout t ?question)
      (bind ?answer (read))
      (if (lexemep ?answer) 
          then (bind ?answer (lowcase ?answer))))
   ?answer)


(deffunction yes-or-no-p (?question)
   (bind ?response (ask-question ?question yes no y n))
   (if (or (eq ?response yes) (eq ?response y))
       then yes 
       else no))



;1. Question about the computer turning on



(defrule question-computer-on ""
   (not (computer-on ?))   
   (not (repair ?))        
   =>
   (assert (computer-on (yes-or-no-p "Does the computer turn on? (yes/no): ")))) 

(defrule computer-on-no ""
   (computer-on no)         
   (not (repair ?))         
   =>
   (assert (repair "Check the computer's power connection. Make sure the power button works. Check the power supply."))) 




;2. Question about the LED indicators



(defrule question-led-indicators ""
   (computer-on yes)        
   (not (led-indicators-on ?))        
   (not (repair ?))         
   =>
   (assert (led-indicators-on (yes-or-no-p "Do the LED indicators on the computer case turn on? (yes/no): ")))) 

(defrule led-indicators-no ""
   (led-indicators-on no)    
   (not (repair ?))          
   =>
   (assert (repair "Check the power connection, the case, or the power button."))) 




;3. Question about the fan



(defrule question-fan ""
   (computer-on yes)        
   (not (fan-ok ?))         
   (not (repair ?))         
   =>
   (assert (fan-ok (yes-or-no-p "Is the cooling fan working? (yes/no): ")))) 

(defrule fan-no ""
   (fan-ok no)             
   (not (repair ?))        
   =>
   (assert (repair "Check the fan connection, clean dust, or replace the fan."))) 




;4. Question about the operating system loading



(defrule question-os-loading ""
   (computer-on yes)        
   (not (os-loads ?))       
   (not (repair ?))         
   =>
   (assert (os-loads (yes-or-no-p "Does the operating system load? (yes/no): ")))) 

(defrule os-loading-no ""
   (os-loads no)             
   (not (repair ?))          
   =>
   (assert (repair "Check if the operating system is installed correctly. Try booting into safe mode. Check for disk errors."))) 




;5. Question about peripherals connection



(defrule question-peripherals ""
   (computer-on yes)         
   (not (peripherals-connected ?))  
   (not (repair ?))          
   =>
   (assert (peripherals-connected (yes-or-no-p "Are all peripherals (keyboard, mouse, monitor, etc.) connected? (yes/no): ")))) 

(defrule peripherals-no ""
   (peripherals-connected no)  
   (not (repair ?))            
   =>
   (assert (repair "Check the connection of peripherals. Ensure all cables are securely connected and functional."))) 



;6. Question about the hard drive status



(defrule question-hard-drive ""
   (computer-on yes)         
   (not (hard-drive-ok ?))   
   (not (repair ?))          
   =>
   (assert (hard-drive-ok (yes-or-no-p "Is the hard drive functioning properly? (yes/no): ")))) 

(defrule hard-drive-no ""
   (hard-drive-ok no)        
   (not (repair ?))          
   =>
   (assert (repair "Check the hard drive for errors. Consider running a disk check or replacing the drive."))) 




;7. Question about network connection



(defrule question-network ""
   (computer-on yes)         
   (not (network-connected ?)) 
   (not (repair ?))          
   =>
   (assert (network-connected (yes-or-no-p "Is the computer connected to the network? (yes/no): ")))) 

(defrule network-no ""
   (network-connected no)    
   (not (repair ?))          
   =>
   (assert (repair "Check the network cable, Wi-Fi connection, or router. Ensure that the network settings are correct."))) 




;8. Question about the monitor's status




(defrule question-monitor ""
   (computer-on yes)         
   (not (monitor-ok ?))   
   (not (repair ?))          
   =>
   (assert (monitor-ok (yes-or-no-p "Is the monitor working properly? (yes/no): ")))) 

(defrule monitor-no ""
   (monitor-ok no)          
   (not (repair ?))          
   =>
   (assert (repair "Check the monitor connection, power supply, and cable. Make sure the display is working."))) 



;9. Question about sound functionality



(defrule question-sound ""
   (computer-on yes)         
   (not (sound-ok ?))   
   (not (repair ?))          
   =>
   (assert (sound-ok (yes-or-no-p "Is the sound working on the computer? (yes/no): ")))) 

(defrule sound-no ""
   (sound-ok no)          
   (not (repair ?))          
   =>
   (assert (repair "Check the sound drivers, speakers, and sound card. Ensure that the audio settings are correct."))) 



;10. Question about the keyboard's functionality



(defrule question-keyboard ""
   (computer-on yes)         
   (not (keyboard-ok ?))   
   (not (repair ?))          
   =>
   (assert (keyboard-ok (yes-or-no-p "Is the keyboard working properly? (yes/no): ")))) 

(defrule keyboard-no ""
   (keyboard-ok no)          
   (not (repair ?))          
   =>
   (assert (repair "Check the keyboard connection, drivers, or try a different keyboard."))) 



;11. Question about the mouse's functionality



(defrule question-mouse ""
   (computer-on yes)         
   (not (mouse-ok ?))   
   (not (repair ?))          
   =>
   (assert (mouse-ok (yes-or-no-p "Is the mouse working properly? (yes/no): ")))) 

(defrule mouse-no ""
   (mouse-ok no)          
   (not (repair ?))          
   =>
   (assert (repair "Check the mouse connection, try another USB port, or replace the mouse."))) 



;12. Question about the power supply



(defrule question-power-supply ""
   (computer-on yes)         
   (not (power-supply-ok ?))   
   (not (repair ?))          
   =>
   (assert (power-supply-ok (yes-or-no-p "Is the power supply functioning properly? (yes/no): ")))) 

(defrule power-supply-no ""
   (power-supply-ok no)        
   (not (repair ?))          
   =>
   (assert (repair "Check the power supply unit for issues or consider replacing it."))) 




;13. Question about software issues
(defrule question-software ""
   (computer-on yes)         
   (not (software-ok ?))   
   (not (repair ?))          
   =>
   (assert (software-ok (yes-or-no-p "Is the software running without issues? (yes/no): ")))) 

(defrule software-no ""
   (software-ok no)        
   (not (repair ?))          
   =>
   (assert (repair "Check for software updates or reinstall the operating system if necessary."))) 



;14. Question about overheating




(defrule question-overheating ""
   (computer-on yes)         
   (not (overheating ?))   
   (not (repair ?))          
   =>
   (assert (overheating (yes-or-no-p "Is the computer overheating? (yes/no): ")))) 

(defrule overheating-no ""
   (overheating no)        
   (not (repair ?))          
   =>
   (assert (repair "Check the cooling system, fans, and clean dust from inside the case."))) 



;15. Question about performance



(defrule question-performance ""
   (computer-on yes)         
   (not (performance-ok ?))   
   (not (repair ?))          
   =>
   (assert (performance-ok (yes-or-no-p "Is the computer performing well? (yes/no): ")))) 

(defrule performance-no ""
   (performance-ok no)        
   (not (repair ?))          
   =>
   (assert (repair "Check CPU and memory load, remove unnecessary programs, run a virus scan, or defragment the hard drive."))) 



;16. System banner



(defrule system-banner ""
  (declare (salience 10))   
  =>
  (printout t crlf crlf)
  (printout t "PC diagnostic")
  (printout t crlf crlf))



; 



(defrule print-repair ""
  (declare (salience 10))   
  (repair ?item)            
  =>
  (printout t crlf crlf)
  (printout t "Suggested Repair:")
  (format t "  %s%n%n%n" ?item))  
