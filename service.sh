#!/bin/bash
systemctl | grep $1 | awk '{print $4}'
