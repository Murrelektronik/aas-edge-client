# Edge Device Management with Asset Administration Shell Connector

This repository contains an implementation of an edge device application, which allows sychronization of edge device status and configuration 
with AAS infrastructure. The app was installed on multiple edge devices from different vendors in Lab Network Industry 4.0 (LNI4.0) testbed 
to demonstrate cross-vendor edge management system using an AAS infrastructure as a central device information point at the HMI 2024 [1].

The application includes two part, a backend and a frontend. The backend collects device information by interacting with various device data sources 
and periodically synchronizes these information with AAS servers using AAS API v3. It also exposes REST API for the frontend component to allow 
changing certain device configuration and showing real-time device status information.

The source code is licenced under Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International
Public License.

Publisher: Murrelektronik GmbH

Authors and Contributors: 
* Markus Rentschler, Xuan-Thuy Dang, Manh Linh Phan and Pham Minh Khai Hoang, Murrelektronik GmbH

[1] https://lni40.de/wp-content/uploads/2024/04/20240422_LNI40_EdgeManagementDemonstrator_v1.3.pdf

