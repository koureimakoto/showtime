using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Player : MonoBehaviour
{

    public float speed = 0f;
    public float jump_force;
    public int   layer_ground = 6;

    private bool single_jumping = true;
    private bool double_jumping = false;

 
    private Rigidbody2D Rig;
    private Animator   Anim;
 
    // Start is called before the first frame update
    void Start() {   
        Rig = GetComponent<Rigidbody2D>();
        Anim= GetComponent<Animator>();
    }

    // Update is called once per frame
    void Update() {
        Move();
        Jump();
    }

    void EnableSingleJump() {
        single_jumping = true;
    }

    void EnableDoubleJump() {
        double_jumping = true;
    }

    void DisableSingleJump() {
        single_jumping = false;
    }

    void DisableDoubleJump() {
        double_jumping = false;
    }

    void Move() {
        float x_axis_input = Input.GetAxis("Horizontal");
        
        // Economia de Recurso, se não for necessário já sai
        if(x_axis_input == 0.0f) {
            Anim.SetBool("walk", false);
            return;
        }
        
        Anim.SetBool("walk", true);

        // Calcular a movimentacao
        float delta        = Time.deltaTime * speed;
        Vector3 mov        = new Vector3(x_axis_input, 0f, 0f);
    
        // Fe movimentacao para direita
        if(x_axis_input > 0f) 
            transform.eulerAngles = new Vector3(0f, 0f,0f);
        else 
        if(x_axis_input < 0f)
            transform.eulerAngles.y = new Vector3(0f, 180f , 0f);

        Debug.Log(new Vector3(0f, 180f , 0f));
        transform.position += mov * delta;
    }

    void ExecJump() {
        Vector2 jump = new Vector2(0f, jump_force);
        Rig.AddForce(
            jump,
            ForceMode2D.Impulse
        );         
    }

    void Jump() {
        bool y_axis_input = Input.GetButtonDown("Jump");
        if(y_axis_input) { 
            if(single_jumping) {
                DisableSingleJump();
                EnableDoubleJump();
                ExecJump();
            } else
            if(double_jumping) {
                DisableDoubleJump();
                ExecJump();
            }
        }     
    }

    bool checkLayerCollision(Collision2D collision) {
        return collision.gameObject.layer == layer_ground;
    }

    void OnCollisionEnter2D(Collision2D collision) {
        if(checkLayerCollision(collision)) {
            if(collision.GetContact(0).normal.y > 0f) 
                    EnableSingleJump();
        }
    }

    
    void OnCollisionExit2D(Collision2D collision) {
        if(checkLayerCollision(collision)) {
        }
    }
}
