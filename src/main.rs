use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Debug)]
struct Approval {
    id: Uuid,
    level: u8,
    approved: bool,
    approver_id: Option<String>,
    entity_id: String,
    requires_approval_from: Vec<Rc<RefCell<Approval>>>,
}

impl Approval {
    fn new(level: u8, entity_id: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            id: Uuid::new_v4(),
            level,
            approved: false,
            approver_id: None,
            entity_id: entity_id.to_string(),
            requires_approval_from: Vec::new(),
        }))
    }

    fn approve(&mut self, approver_id: String, approver_level: u8) -> Result<(), ()> {
        if self.level < approver_level {
            return Err(());
        }

        for approval in &self.requires_approval_from {
            if !approval.borrow().approved {
                return Err(());
            }
        }

        self.approved = true;
        self.approver_id = Some(approver_id);
        Ok(())
    }

    fn require_approval_from(&mut self, approvals: Vec<Rc<RefCell<Approval>>>) {
        self.requires_approval_from.extend(approvals);
    }
}

fn main() {
    let entity_id = String::from("entity1");

    // Create the level 1 approval
    let level_1_approval = Approval::new(1, &entity_id);

    // Create the level 2 approval
    let level_2_approval = Approval::new(2, &entity_id);

    // Set level 1 approval as a requirement for level 2 approval
    level_2_approval
        .borrow_mut()
        .require_approval_from(vec![Rc::clone(&level_1_approval)]);

    // Approve level 1 approval first
    // {
    //     let mut lvl_1 = level_1_approval.borrow_mut();
    //     let success1 = lvl_1.approve(String::from("approver_level_1"), 1);
    //     if let Err(()) = success1 {
    //         println!("Failed to approve Level 1 Approval! id: {}", lvl_1.id);
    //     } else {
    //         println!("Successfully approved Level 1 Approval! id: {}", lvl_1.id);
    //     }
    // }

    // Now approve level 2 approval
    {
        let mut lvl_2 = level_2_approval.borrow_mut();
        let success2 = lvl_2.approve(String::from("approver_level_2"), 2);
        if let Err(()) = success2 {
            println!("Failed to approve Level 2 Approval! id: {}", lvl_2.id);
        } else {
            println!("Successfully approved Level 2 Approval! id: {}", lvl_2.id);
        }
    }

    // Print the approvals for debugging
    println!("{:#?}", level_1_approval);
    println!("{:#?}", level_2_approval);
}
