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

    private LayerMask mask;
 
    private Rigidbody2D Rig;
    private Animator   Anim;

    private BoxCollider2D Collider;
 
    // Start is called before the first frame update
    void Start() {   
        Rig      = GetComponent<Rigidbody2D>();
        Anim     = GetComponent<Animator>();
        Collider = GetComponent<BoxCollider2D>();
        mask     = LayerMask.GetMask("Ground");
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

        // Calcular a movimentacao
        float delta        = Time.deltaTime * speed;
        Vector3 mov        = new Vector3(x_axis_input, 0f, 0f);
    
        if(single_jumping && !double_jumping)
            Anim.SetBool("walk", true);

        // Realiza animacao de movimentacao para direita e esquerda
        if(x_axis_input > 0f) 
            transform.eulerAngles = new Vector3(0f, 0f,0f);
        else 
        if(x_axis_input < 0f)
            transform.eulerAngles = new Vector3(0f, 180f , 0f);

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
            if(IsGrounded() && single_jumping) {
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

    private bool IsGrounded() {
        // Se eu entendi corretamente, ele pega a diferença distancia do centro em realação ao seu tamanho.
        // https://www.youtube.com/watch?v=ptvK4Fp5vRY&ab_channel=CodeMonkey
        RaycastHit2D RayCast = Physics2D.BoxCast(
            Collider.bounds.center,  // Center                      |
            Collider.bounds.size,    // Half the size of the Box    | -- Vector3
            0f,                      // Direction                   |
            Vector2.down,            // Info
            .1f,
            mask
        );
        
        return(
            RayCast.collider != null
        );
    }


    void OnCollisionStay2D(Collision2D collision) {
        if(checkLayerCollision(collision)) {
            //if(collision.GetContact(0).normal.y > 0.0f) {
                Anim.SetBool("jump", false);
                EnableSingleJump();
            //}
        }
    }

    
    void OnCollisionExit2D(Collision2D collision) {
        if(checkLayerCollision(collision)) {
            Anim.SetBool("jump", true);
            DisableSingleJump();
        }
    }
}
