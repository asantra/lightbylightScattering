import os, sys
import math, time
import h5py
import subprocess
from array import array
import numpy as np
import ROOT
from ROOT import TFile, TTree, TLorentzVector
import glob
import subprocess
from subprocess import call
import argparse



meMeV = 0.5109989461 ## MeV
meGeV = meMeV/1000.
MeV2GeV = 1./1000.

def main():
    photon = False
    parser = argparse.ArgumentParser(description='Code to get root files from h5')
    parser.add_argument('-x', action="store", dest="xi", type=float, default=3.0)
    args = parser.parse_args()
    xiInput = args.xi
    
    storage   = "rootFiles"

    p         = subprocess.Popen("mkdir -p "+storage, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    out, err  = p.communicate()


    tf = TFile( storage+'/raw_lightbylight_xi'+str(xiInput)+'.root', 'recreate' )

    tt_out    = TTree( 'tt','tt' )
    vx_out    = ROOT.std.vector( float )()
    vy_out    = ROOT.std.vector( float )()
    vz_out    = ROOT.std.vector( float )()
    px_out    = ROOT.std.vector( float )()
    py_out    = ROOT.std.vector( float )()
    pz_out    = ROOT.std.vector( float )()
    eta_out   = ROOT.std.vector( float )()
    theta_out = ROOT.std.vector( float )()
    phi_out   = ROOT.std.vector( float )()
    E_out     = ROOT.std.vector( float )()
    pdgId_out = ROOT.std.vector( int )()
    mpid_out  = ROOT.std.vector( str )()
    wgt_out   = ROOT.std.vector( float )()
    time_out  = ROOT.std.vector( float )()
    xi_out    = ROOT.std.vector( float )()
    tt_out.Branch('vx', vx_out)
    tt_out.Branch('vy', vy_out)
    tt_out.Branch('vz', vz_out)
    tt_out.Branch('px', px_out)
    tt_out.Branch('py', py_out)
    tt_out.Branch('pz', pz_out)
    tt_out.Branch('eta', eta_out)
    tt_out.Branch('theta', theta_out)
    tt_out.Branch('phi', phi_out)
    tt_out.Branch('E',  E_out)
    tt_out.Branch('wgt',  wgt_out)
    tt_out.Branch('pdgId',pdgId_out)
    tt_out.Branch('mpid',mpid_out)
    tt_out.Branch('time',time_out)
    tt_out.Branch('xi',xi_out)

    
    fIns = glob.glob("xi"+str(xiInput)+"/*.h5")
    #print(fIns)   
    photonNumberList = []
    ##### work only on the events having same order of tracks as that of the highest tracked event
    for name in fIns:
        #### input file
        fIn = h5py.File(name, 'r')
        id_value_photon = fIn['final-state/photon']['id'][()]
        photonNumberList.append(len(id_value_photon))

    #### sort the list in maximum to minimum
    photonNumberList.sort(reverse=True)
    
    ### take the first 10 highest photon events
    highestPhotonEvents = float(photonNumberList[0])

    for name in fIns:
        ### clear output tree branches
        mpid_out.clear()
        pdgId_out.clear()
        wgt_out.clear()
        vx_out.clear()
        vy_out.clear()
        vz_out.clear()
        px_out.clear()
        py_out.clear()
        pz_out.clear()
        eta_out.clear()
        theta_out.clear()
        phi_out.clear()
        E_out.clear()
        xi_out.clear()
        time_out.clear()
        
        #### input file
        fIn = h5py.File(name, 'r')
        print("reading: ",name)
                
        ### photons
        photonNumber = 0
        id_value_photon       = fIn['final-state/photon']['id'][()]
        parentid_value_photon = fIn['final-state/photon']['parent_id'][()]
        momentum_value_photon = fIn['final-state/photon']['momentum'][()]
        position_value_photon = fIn['final-state/photon']['position'][()]
        weight_value_photon   = fIn['final-state/photon']['weight'][()]
        # print("this file has ",len(id_value_photon)," photons")
        if(highestPhotonEvents>50):
            if len(id_value_photon) < highestPhotonEvents/2.0: 
                print("This file ",name," has very few photons ",len(id_value_photon), " ---- NOT PROCESSING")
                continue
        else:
            if len(id_value_photon) < highestPhotonEvents/10.0: 
                print("This file ",name," has very few photons ",len(id_value_photon), " ---- NOT PROCESSING")
                continue

        for j in range(0, len(id_value_photon)):
            vx0    = position_value_photon[j][0]*1.e-1 ## mm to cm
            vy0    = position_value_photon[j][1]*1.e-1 ## mm to cm
            vz0    = position_value_photon[j][2]*1.e-1 ## mm to cm
            t0     = position_value_photon[j][3]
            Energy = momentum_value_photon[j][0]
            px0    = momentum_value_photon[j][1]
            py0    = momentum_value_photon[j][2]
            pz0    = momentum_value_photon[j][3]

            photonVec = TLorentzVector() 
            photonVec.SetPxPyPzE(px0, py0, pz0, Energy)

            eta0   = photonVec.Eta()
            theta0 = photonVec.Theta()
            phi0   = photonVec.Phi()

            pdgId0 = 22
            wgt0   = weight_value_photon[j]
            MP_ID  = str(id_value_photon[j])+"_"+str(pdgId0)
            xi0    = xiInput
            mpid_out.push_back(str(MP_ID))
            wgt_out.push_back(wgt0)  
            pdgId_out.push_back(int(pdgId0))  
            vx_out.push_back(vx0)
            vy_out.push_back(vy0)
            vz_out.push_back(vz0)
            px_out.push_back(px0)
            py_out.push_back(py0)
            pz_out.push_back(pz0)
            eta_out.push_back(eta0)
            theta_out.push_back(theta0)
            phi_out.push_back(phi0)
            E_out.push_back(Energy)
            time_out.push_back(t0)
            xi_out.push_back(xi0)
            photonNumber += 1

        electronNumber = 0
        if(True):
            ### electrons are only collected for g+laser
            id_value_electron       = fIn['final-state/electron']['id'][()]
            # print("this file has ",len(id_value_electron)," electrons")
            parentid_value_electron = fIn['final-state/electron']['parent_id'][()]
            momentum_value_electron = fIn['final-state/electron']['momentum'][()]
            position_value_electron = fIn['final-state/electron']['position'][()]
            weight_value_electron   = fIn['final-state/electron']['weight'][()]
            for j in range(0, len(id_value_electron)):
                vx0    = position_value_electron[j][0]*1.e-1 ## mm to cm
                vy0    = position_value_electron[j][1]*1.e-1 ## mm to cm
                vz0    = position_value_electron[j][2]*1.e-1 ## mm to cm
                t0     = position_value_electron[j][3]
                Energy = momentum_value_electron[j][0]
                px0    = momentum_value_electron[j][1]
                py0    = momentum_value_electron[j][2]
                pz0    = momentum_value_electron[j][3]

                electronVec = TLorentzVector()
                electronVec.SetPxPyPzE(px0, py0, pz0, Energy)

                eta0   = electronVec.Eta()
                theta0 = electronVec.Theta()
                phi0   = electronVec.Phi()


                pdgId0 = 11
                wgt0   = weight_value_electron[j]
                MP_ID  = str(id_value_electron[j])+"_"+str(pdgId0)
                xi0    = xiInput
                mpid_out.push_back(str(MP_ID))
                wgt_out.push_back(wgt0)  
                pdgId_out.push_back(int(pdgId0))  
                vx_out.push_back(vx0)
                vy_out.push_back(vy0)
                vz_out.push_back(vz0)
                px_out.push_back(px0)
                py_out.push_back(py0)
                pz_out.push_back(pz0)
                eta_out.push_back(eta0)
                theta_out.push_back(theta0)
                phi_out.push_back(phi0)
                E_out.push_back(Energy)
                time_out.push_back(t0)
                xi_out.push_back(xi0)
                electronNumber += 1
            

        tt_out.Fill()
        print("electrons ", electronNumber, " photons ", photonNumber, " in file ", name)
        
    tt_out.Write()
    tf.Write()
    tf.Write()
    tf.Close()


if __name__=="__main__":
    intime = time.time()
    main()
    print("----- the time taken ", time.time() - intime, " s")
